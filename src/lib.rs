//! Pure Rust Bonding Curve quoting SDK intended for Jupiter AMM integration.
//!
//! The crate is designed to be:
//!
//! - deterministic
//! - forkable
//! - independent from RPC or network access
//! - thin enough that the Jupiter adapter only has to parse state and build metas

mod accounts;
mod errors;
mod fees;
mod math;
mod quote;
mod state;

pub use accounts::{
    build_swap_account_metas, config_address, event_authority, pool_authority,
    referral_account_meta, SwapAccountMetasParams, BONDING_CURVE_LABEL,
    BONDING_CURVE_PROGRAM_ID, BONDING_CURVE_SWAP_ACCOUNTS_LEN,
};
pub use errors::QuoteError;
pub use fees::{
    calculate_fees, calculate_market_cap, get_fee_rates, get_referral_fee_rate, FeeBreakdown,
};
pub use quote::{
    quote, quote_base_to_quote, quote_for_mints, quote_quote_to_base, supports_mints,
    trade_direction_from_mints, QuoteRequest, QuoteResult, TradeDirection,
};
pub use state::{PoolSnapshot, BONDING_CURVE_POOL_DISCRIMINATOR, WSOL_MINT};
