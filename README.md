# Bonding Curve SDK

Pure Rust quoting SDK for the Bonding Curve venue.

Design goals:

- easy for Jupiter to fork and maintain
- no network calls
- no RPC dependency
- deterministic quote behavior from account snapshot input only
- thin adapter layer in `jupiter-core`
- standalone-repo friendly packaging

Current scope:

- deserialize Bonding Curve pool state into `PoolSnapshot`
- quote both directions with on-chain fee semantics
- expose venue metadata and PDA helpers
- build swap account metas in original contract ABI order
- stay free of Jupiter-specific trait types

Main exports:

- `PoolSnapshot`
- `PoolSnapshot::try_from_account_data`
- `TradeDirection`
- `QuoteRequest`
- `QuoteResult`
- `quote`
- `quote_for_mints`
- `trade_direction_from_mints`
- `supports_mints`
- `calculate_market_cap`
- `calculate_fees`
- `BONDING_CURVE_PROGRAM_ID`
- `BONDING_CURVE_LABEL`
- `pool_authority`
- `config_address`
- `event_authority`
- `SwapAccountMetasParams`
- `build_swap_account_metas`

Minimal Jupiter adapter flow:

```rust
use bonding_curve_sdk::{quote_for_mints, PoolSnapshot, WSOL_MINT};

let snapshot = PoolSnapshot::try_from_account_data(&pool_account.data)?;
let sdk_quote = quote_for_mints(
    &snapshot,
    input_mint,
    output_mint,
    amount_in,
    false,
)?;

let jupiter_quote = jupiter_amm_interface::Quote {
    in_amount: amount_in,
    out_amount: sdk_quote.amount_out,
    fee_amount: sdk_quote.fee_amount,
    fee_mint: sdk_quote.fee_mint,
    fee_pct: rust_decimal::Decimal::from(sdk_quote.fee_amount)
        / rust_decimal::Decimal::from(amount_in),
};
```

Current referral status:

- no-referral quote is supported
- referral-aware quote follows the current on-chain fee split rules
- the current Jupiter adapter still defaults to no-referral quote because the `Amm` quote surface does not carry referral presence
- swap execution can still pass a referral token account separately through `SwapParams::quote_mint_to_referrer`

What stays outside this crate:

- `impl Amm for BondingCurveAmm`
- `QuoteParams` / `SwapParams` mapping
- placeholder account policy for optional referral accounts
- route execution / CPI plumbing
- loader registration

Standalone repo checklist:

- copy `bonding-curve-sdk/src/*`
- copy this `README.md`
- copy `LICENSE`
- copy `CHANGELOG.md`
- copy [extraction_checklist.md](./extraction_checklist.md)
- copy `handoff/*` if you want to hand Jupiter a ready-made adapter template
- fill `repository`, `homepage`, and `documentation` in `Cargo.toml`
- review the `LICENSE` copyright holder string before handing the repo to Jupiter
- add CI before handing the repo to Jupiter

This crate is intended to satisfy Jupiter's DEX integration expectation that the SDK remains forkable, deterministic, and free of runtime network access.
