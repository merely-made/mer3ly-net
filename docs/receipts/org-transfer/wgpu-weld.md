# wgpu-weld organization transfer

Verified at `2026-07-29T21:35:26Z`.

## Transfer

- Source: `mark-ik/wgpu-weld`
- Target: `merely-made/wgpu-weld`
- Pre-transfer head: `5e46c0c8a3e3a1599e9e4e3a3725dd4f53b50f15`
- Post-transfer head: `481911ed4000cdddcfe193d8ead412bb60fb71b2`
- Default branch: `main`
- Visibility: public
- Local origin: `https://github.com/merely-made/wgpu-weld.git`

The pre-transfer audit found no Actions workflow, Pages site, package,
repository ruleset, secret, variable, deploy key, webhook, or release.

## Redirects

- The former web URL redirects to `https://github.com/merely-made/wgpu-weld`.
- The former API slug resolves as `merely-made/wgpu-weld`.
- `git ls-remote` returns the same `main` head through the former and canonical
  Git URLs.

## Canonicalization

- wgpu-weld commit `481911e` updates package metadata, documentation,
  `grafting` dependency URLs, and its tracked lock.
- Genet commit `4ca5b45f0aa` updates its public weld-engine documentation.
- The lock resolves one `grafting` source at
  `merely-made/wgpu-graft` commit `020b800`.

## Verification

- Full `cargo metadata --format-version 1`
- `cargo test -p welding --lib`
- One welding library test passes.
- Source scan: zero tracked `mark-ik/wgpu-weld` references
