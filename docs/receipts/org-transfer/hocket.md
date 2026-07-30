# Hocket organization transfer

Verified at `2026-07-30T00:22:56Z`.

## Transfer

- Source: `mark-ik/hocket`
- Target: `merely-made/hocket`
- Pre-transfer head: `d89c862c7ade5de308eb17d1b5d0eea60f5b79f8`
- Post-transfer head: `cfe47b07bb4a81520b5414528ec0557d8abc8dcf`
- Default branch: `main`
- Visibility: public
- Local origin: `https://github.com/merely-made/hocket.git`

The pre-transfer audit found no Actions workflow, Pages site, package,
repository ruleset, secret, variable, deploy key, webhook, or release.
Post-transfer Actions remain enabled for all actions, with zero repository
secrets and variables.

## Redirects

- The former web URL redirects to `https://github.com/merely-made/hocket`.
- The former API slug resolves as `merely-made/hocket`.
- `git ls-remote` returns the same `main` head through the former and canonical
  Git URLs.

## Canonicalization

- Hocket commit `cfe47b07bb4` updates package metadata, its Hocket feed
  identity, and the Woodshed dependency lock.
- Mere commit `d18cfdf7c26` canonicalizes its public Hocket links and luggage
  feed fixtures.
- Tracked repositories outside the historical Mer3ly receipts contain zero
  `mark-ik/hocket` references.

## Verification

- Clean locked metadata resolves Woodshed at `5011b91a`.
- Hocket model tests: 38 passed
- `cargo check -p hocket-genet`
- Mere luggage: 34 library tests, 1 binary test, and 1 doctest passed
