# Bonding Curve SDK Extraction Checklist

Use this checklist when moving `bonding-curve-sdk` into its own repository for Jupiter review or handoff.

## Files To Copy

- `bonding-curve-sdk/Cargo.toml`
- `bonding-curve-sdk/README.md`
- `bonding-curve-sdk/LICENSE`
- `bonding-curve-sdk/CHANGELOG.md`
- `bonding-curve-sdk/handoff/bonding_curve_amm.rs`
- `bonding-curve-sdk/handoff/integration_notes.md`
- `bonding-curve-sdk/src/accounts.rs`
- `bonding-curve-sdk/src/errors.rs`
- `bonding-curve-sdk/src/fees.rs`
- `bonding-curve-sdk/src/lib.rs`
- `bonding-curve-sdk/src/math.rs`
- `bonding-curve-sdk/src/quote.rs`
- `bonding-curve-sdk/src/state.rs`

## Cargo Metadata To Fill Before Delivery

- `repository`
- `homepage`
- `documentation`
- `authors` if you want them visible in crates.io metadata

## Repository Files To Add

- `LICENSE`
- `CHANGELOG.md`
- `.gitignore`
- CI workflow that runs:
  - `cargo fmt --check`
  - `cargo test`

Review before delivery:

- confirm the copyright holder in `LICENSE`
- confirm the version in `CHANGELOG.md`

## README Expectations

Make sure the standalone repo README still explains:

- what the crate does
- what stays outside the crate
- current referral quote semantics
- how Jupiter should call the SDK from an `Amm` adapter
- which commands validate the crate locally

## Validation Commands

Run these before handing the repo to Jupiter:

```bash
cargo fmt --check
cargo test
```

## Handoff Notes For Jupiter

Include these points in the repo description or handoff message:

- the crate is deterministic and does not perform network calls
- it owns Bonding Curve quote math, fee logic, pool parsing, venue metadata, and ABI-order account metas
- it does not own Jupiter `Amm` trait glue
- referral-aware quote is supported in the SDK, but current Jupiter `Amm::quote()` integration may still choose a no-referral policy if referrer context is absent
