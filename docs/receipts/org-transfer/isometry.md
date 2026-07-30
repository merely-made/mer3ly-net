# Isometry organization transfer

Verified at `2026-07-30T00:22:56Z`.

## Transfer

- Source: `mark-ik/isometry`
- Target: `merely-made/isometry`
- Pre-transfer head: `885d7dda41c973732e4946687ce2b7323daa3838`
- Post-transfer head: `00b0a78b51cb4116f9f8564d3925beb77002d62e`
- Default branch: `main`
- Visibility: public
- Local origin: `https://github.com/merely-made/isometry.git`

The pre-transfer audit found no Actions workflow, Pages site, package,
repository ruleset, secret, variable, deploy key, webhook, or release.
Post-transfer Actions remain enabled for all actions, with zero repository
secrets and variables.

## Redirects

- The former web URL redirects to `https://github.com/merely-made/isometry`.
- The former API slug resolves as `merely-made/isometry`.
- `git ls-remote` returns the same `main` head through the former and canonical
  Git URLs.

## Canonicalization

- Isometry commit `00b0a78b51c` updates its package repository metadata.
- Tracked repositories outside the historical Mer3ly receipts contain zero
  `mark-ik/isometry` references.

## Verification

- `cargo metadata --format-version 1 --no-deps`
- `cargo check --workspace --all-features --all-targets --locked --offline`
