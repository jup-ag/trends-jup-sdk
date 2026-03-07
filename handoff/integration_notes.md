# Jupiter Integration Notes

This folder is for the future standalone `bonding-curve-sdk` repository handoff.

## What To Give Jupiter

- the SDK crate from `bonding-curve-sdk/src/*`
- this adapter template: [bonding_curve_amm.rs](./bonding_curve_amm.rs)
- the extraction checklist: [../extraction_checklist.md](../extraction_checklist.md)

## Expected Integration Shape

Jupiter should:

1. depend on `bonding-curve-sdk`
2. copy `bonding_curve_amm.rs` into their AMM integration repo
3. register `BondingCurveAmm` in their AMM loader / program-id map
4. wire `Swap::BondingCurve` to an execution path that recognizes the variant
5. run snapshot-based tests and route execution tests

## What The Template Assumes

- `Swap::BondingCurve` already exists in the target repo
- `jupiter_amm_interface` is available
- the target repo wants a no-referral quote policy until quote params carry referrer context

## Why Quote Uses No-Referral Policy

The SDK supports referral-aware quote math.

The adapter template still defaults `Amm::quote()` to no-referral because `QuoteParams`
does not currently include referrer presence. Execution can still pass referral accounts
through `SwapParams::quote_mint_to_referrer`.

## What Still Stays Outside The SDK

- loader registration
- `Swap::BondingCurve` enum ownership
- execution-layer CPI handling
- route-plan execution tests

## Recommended Follow-Up In Jupiter Repo

- add `program_dependencies()` to test harness expectations if useful
- add a snapshot fixture for one live Bonding Curve pool
- add one quote test per direction
- add one swap-account-metas shape test
- add execution tests once the aggregator binary supports the variant
