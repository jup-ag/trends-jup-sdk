use anyhow::{ensure, Result};
use jupiter_amm_interface::{
    single_program_amm, try_get_account_data, AccountMap, Amm, AmmContext, KeyedAccount, Quote,
    QuoteParams, SingleProgramAmm, Swap, SwapAndAccountMetas, SwapMode, SwapParams,
};
use rust_decimal::Decimal;
use solana_pubkey::Pubkey;

use crate::{
    build_swap_account_metas, quote_for_mints, supports_mints, PoolSnapshot,
    SwapAccountMetasParams, BONDING_CURVE_LABEL, BONDING_CURVE_PROGRAM_ID,
    BONDING_CURVE_SWAP_ACCOUNTS_LEN, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID, WSOL_MINT,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum QuoteReferralPolicy {
    NoReferralContextInQuoteParams,
}

#[derive(Clone)]
pub struct BondingCurveAmm {
    key: Pubkey,
    state: PoolSnapshot,
}

single_program_amm!(
    BondingCurveAmm,
    BONDING_CURVE_PROGRAM_ID,
    BONDING_CURVE_LABEL
);

impl Amm for BondingCurveAmm {
    fn from_keyed_account(keyed_account: &KeyedAccount, _amm_context: &AmmContext) -> Result<Self> {
        ensure!(
            keyed_account.account.owner == BONDING_CURVE_PROGRAM_ID,
            "Unexpected owner for bonding curve pool"
        );

        Ok(Self {
            key: keyed_account.key,
            state: PoolSnapshot::try_from_account_data(&keyed_account.account.data)?,
        })
    }

    fn label(&self) -> String {
        BONDING_CURVE_LABEL.to_string()
    }

    fn program_id(&self) -> Pubkey {
        BONDING_CURVE_PROGRAM_ID
    }

    fn key(&self) -> Pubkey {
        self.key
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        vec![self.state.base_mint, WSOL_MINT]
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        vec![self.key]
    }

    fn update(&mut self, account_map: &AccountMap) -> Result<()> {
        let pool_account_data = try_get_account_data(account_map, &self.key)?;
        self.state = PoolSnapshot::try_from_account_data(pool_account_data)?;
        Ok(())
    }

    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {
        ensure!(
            quote_params.swap_mode == SwapMode::ExactIn,
            "Bonding curve AMM only supports exact-in quotes"
        );

        self.ensure_supported_pair(quote_params.input_mint, quote_params.output_mint, "AMM")?;

        let sdk_quote = quote_for_mints(
            &self.state,
            quote_params.input_mint,
            quote_params.output_mint,
            quote_params.amount,
            self.quote_has_referral(),
        )?;

        Ok(to_jupiter_quote(quote_params.amount, sdk_quote))
    }

    fn get_swap_and_account_metas(&self, swap_params: &SwapParams) -> Result<SwapAndAccountMetas> {
        self.ensure_supported_pair(
            swap_params.source_mint,
            swap_params.destination_mint,
            "swap",
        )?;

        Ok(SwapAndAccountMetas {
            swap: Swap::MeteoraDynamicBondingCurveSwapWithRemainingAccounts,
            account_metas: build_swap_account_metas(
                &self.state,
                SwapAccountMetasParams {
                    pool: self.key,
                    source_token_account: swap_params.source_token_account,
                    destination_token_account: swap_params.destination_token_account,
                    token_transfer_authority: swap_params.token_transfer_authority,
                    referral_token_account: referral_token_account(swap_params),
                    referral_placeholder: swap_params.placeholder_account_meta(),
                },
            ),
        })
    }

    fn get_accounts_len(&self) -> usize {
        BONDING_CURVE_SWAP_ACCOUNTS_LEN
    }

    fn supports_exact_out(&self) -> bool {
        false
    }

    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        Box::new(self.clone())
    }

    fn program_dependencies(&self) -> Vec<(Pubkey, String)> {
        vec![
            (BONDING_CURVE_PROGRAM_ID, "bonding_curve".to_string()),
            (TOKEN_2022_PROGRAM_ID, "spl_token_2022".to_string()),
            (TOKEN_PROGRAM_ID, "spl_token".to_string()),
        ]
    }

    fn is_active(&self) -> bool {
        self.state.virtual_base_reserve > 0 && self.state.virtual_quote_reserve > 0
    }
}

impl BondingCurveAmm {
    fn ensure_supported_pair(
        &self,
        input_mint: Pubkey,
        output_mint: Pubkey,
        context: &str,
    ) -> Result<()> {
        ensure!(
            supports_mints(&self.state, input_mint, output_mint),
            "Unsupported mint pair for bonding curve {context}"
        );
        Ok(())
    }

    fn quote_has_referral(&self) -> bool {
        match QuoteReferralPolicy::NoReferralContextInQuoteParams {
            QuoteReferralPolicy::NoReferralContextInQuoteParams => false,
        }
    }
}

fn referral_token_account(swap_params: &SwapParams) -> Option<Pubkey> {
    swap_params
        .quote_mint_to_referrer
        .and_then(|quote_mint_to_referrer| quote_mint_to_referrer.get(&WSOL_MINT))
        .copied()
}

fn to_jupiter_quote(in_amount: u64, sdk_quote: crate::QuoteResult) -> Quote {
    let fee_pct = if in_amount == 0 {
        Decimal::ZERO
    } else {
        Decimal::from(sdk_quote.fee_amount) / Decimal::from(in_amount)
    };

    Quote {
        in_amount,
        out_amount: sdk_quote.amount_out,
        fee_amount: sdk_quote.fee_amount,
        fee_mint: sdk_quote.fee_mint,
        fee_pct,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_snapshot() -> PoolSnapshot {
        PoolSnapshot {
            base_mint: Pubkey::new_unique(),
            base_vault: Pubkey::new_unique(),
            quote_vault: Pubkey::new_unique(),
            base_reserve: 1_000_000_000_000_000,
            quote_reserve: 10_000_000_000,
            virtual_base_reserve: 1_000_000_000_000_000,
            virtual_quote_reserve: 20_000_000_000,
        }
    }

    #[test]
    fn adapter_reports_pool_key_for_updates() {
        let amm = BondingCurveAmm {
            key: Pubkey::new_unique(),
            state: sample_snapshot(),
        };

        assert_eq!(amm.get_accounts_to_update(), vec![amm.key]);
    }

    #[test]
    fn adapter_reports_expected_program_dependencies() {
        let amm = BondingCurveAmm {
            key: Pubkey::new_unique(),
            state: sample_snapshot(),
        };

        assert_eq!(
            amm.program_dependencies(),
            vec![
                (BONDING_CURVE_PROGRAM_ID, "bonding_curve".to_string()),
                (TOKEN_2022_PROGRAM_ID, "spl_token_2022".to_string()),
                (TOKEN_PROGRAM_ID, "spl_token".to_string()),
            ]
        );
    }
}
