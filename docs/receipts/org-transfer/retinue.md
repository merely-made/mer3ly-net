# Retinue organization transfer

Verified at `2026-07-29T23:24:55Z`.

## Transfer

- Source: `mark-ik/retinue`
- Target: `merely-made/retinue`
- Pre-transfer head: `6f04bb93e248d08756b0208db9c7b3b397182f79`
- Post-transfer head: `7439a79de4dbdb98292f52906cacc0c7fad024e3`
- Default branch: `main`
- Visibility: public
- Local origin: `https://github.com/merely-made/retinue.git`

The pre-transfer audit found one Actions workflow and no Pages site, package,
repository ruleset, secret, variable, deploy key, webhook, or release.
Post-transfer Actions remain enabled for all actions, with zero repository
secrets and variables.

## Redirects

- The former web URL redirects to `https://github.com/merely-made/retinue`.
- The former API slug resolves as `merely-made/retinue`.
- `git ls-remote` returns the same `main` head through the former and canonical
  Git URLs.

## Canonicalization

- Retinue commit `7439a79de4d` updates 14 tracked package metadata,
  documentation, and source-link files.
- Mere commit `d5af0618cb3` canonicalizes its Retinue dependency and URL-keyed
  patch table.
- Tracked repositories outside the historical Mer3ly receipts contain zero
  `mark-ik/retinue` references.

## Verification

- `cargo metadata --format-version 1 --no-deps`
- `cargo test`: complete default host suite passed, including doctests
- Mere clean locked metadata resolves one Retinue source at `7439a79d`.
