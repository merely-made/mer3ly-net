# Woodshed organization transfer

Verified at `2026-07-30T00:22:56Z`.

## Transfer

- Source: `mark-ik/woodshed`
- Target: `merely-made/woodshed`
- Pre-transfer head: `99eba4aec1fb742980a3c416b714a6c0d32b6fee`
- Post-transfer head: `5011b91ad7dc801ef819fbf0f3681f9ba6e6cd0f`
- Default branch: `main`
- Visibility: public
- Local origin: `https://github.com/merely-made/woodshed.git`

The pre-transfer audit found two Actions workflows and no Pages site, package,
repository ruleset, secret, variable, deploy key, webhook, or release.
Post-transfer Actions remain enabled for all actions, with zero repository
secrets and variables.

## Redirects

- The former web URL redirects to `https://github.com/merely-made/woodshed`.
- The former API slug resolves as `merely-made/woodshed`.
- `git ls-remote` returns the same `main` head through the former and canonical
  Git URLs.

## Canonicalization

- Woodshed commit `5011b91ad7d` updates its root package metadata, workspace
  documentation, and Windows packaging instructions.
- Hocket commit `cfe47b07bb4` moves `audio-primitives` to the canonical
  Woodshed source and pins the transferred Woodshed head.
- Tracked repositories outside the historical Mer3ly receipts contain zero
  `mark-ik/woodshed` references.

## Verification

- `cargo metadata --format-version 1 --no-deps`
- Hocket locked metadata resolves one Woodshed source at `5011b91a`.
- Hocket: 38 model tests and `cargo check -p hocket-genet`
