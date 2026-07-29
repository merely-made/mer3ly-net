# wgpu-scry organization transfer

Verified at `2026-07-29T21:35:26Z`.

## Transfer

- Source: `mark-ik/wgpu-scry`
- Target: `merely-made/wgpu-scry`
- Pre-transfer head: `d8fc221a47e172ebbd1dcb0a87fd37d2e0acce06`
- Post-transfer head: `0af3b54a0d3df3686a28ed99b49f654e4720522c`
- Default branch: `main`
- Visibility: public
- Local origin: `https://github.com/merely-made/wgpu-scry.git`

The pre-transfer audit found two Actions workflows and no Pages site, package,
repository ruleset, secret, variable, deploy key, webhook, or release.

## Redirects

- The former web URL redirects to `https://github.com/merely-made/wgpu-scry`.
- The former API slug resolves as `merely-made/wgpu-scry`.
- `git ls-remote` returns the same `main` head through the former and canonical
  Git URLs.

## Canonicalization

- wgpu-scry commit `0af3b54` updates package metadata, documentation,
  `grafting` dependency URLs, and its tracked lock.
- Genet commit `4ca5b45f0aa` updates the `scrying-engine` dependency and public
  interop documentation.
- The lock resolves one `grafting` source at
  `merely-made/wgpu-graft` commit `020b800`.

## Verification

- Full `cargo metadata --format-version 1`
- `cargo test -p scrying --lib`
- `cargo check -p scrying-engine` in Genet
- Source scan: zero tracked `mark-ik/wgpu-scry` references
