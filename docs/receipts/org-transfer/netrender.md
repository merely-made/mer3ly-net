# Netrender organization transfer

Verified at `2026-07-29T21:35:26Z`.

## Transfer

- Source: `mark-ik/netrender`
- Target: `merely-made/netrender`
- Pre-transfer head: `4b403c47f703af7f114d4ed1e1468edf8b195cb7`
- Post-transfer head: `f20c54627f2fe8bb059a006f83599badbce3ffd5`
- Default branch: `main`
- Visibility: public
- Local origin: `https://github.com/merely-made/netrender.git`

The pre-transfer audit found one Actions workflow and no Pages site, package,
repository ruleset, secret, variable, deploy key, webhook, or release.

## Redirects

- The former web URL redirects to `https://github.com/merely-made/netrender`.
- The former API slug resolves as `merely-made/netrender`.
- `git ls-remote` returns the same `main` head through the former and canonical
  Git URLs.

## Canonicalization

- Netrender commit `f20c54627` updates all package repository metadata.
- Genet commit `4ca5b45f0aa`, Mere commit `48e277d4`, Turnstone commit
  `00600c4`, Woodshed commit `6769012`, Hocket commit `4641fb3`, and Isometry
  commit `0d220eb` canonicalize their Netrender edges.
- Clean Woodshed, Hocket, and Isometry locks contain one Netrender Git source,
  under `merely-made`. Turnstone's ignored local lock has the same identity.

## Verification

- `cargo metadata --format-version 1 --no-deps`
- `cargo check -p netrender`
- Consumer source scan: zero tracked `mark-ik/netrender` references
- Clean-lock checks: Woodshed and Hocket application checks, Turnstone's 154
  library tests, and Isometry's all-features/all-targets workspace check
