# wgpu-graft organization transfer

Verified at `2026-07-29T21:35:26Z`.

## Transfer

- Source: `mark-ik/wgpu-graft`
- Target: `merely-made/wgpu-graft`
- Pre-transfer head: `941c921acfb5a6d05271414479fb072abc058ee3`
- Post-transfer head: `020b800a269dcbd0dbe894c4b84542d44d207f28`
- Default branch: `main`
- Visibility: public
- Local origin: `https://github.com/merely-made/wgpu-graft.git`

The pre-transfer audit found four Actions workflows and no Pages site, package,
repository ruleset, secret, variable, deploy key, webhook, or release.

## Redirects

- The former web URL redirects to `https://github.com/merely-made/wgpu-graft`.
- The former API slug resolves as `merely-made/wgpu-graft`.
- `git ls-remote` returns the same `main` head through the former and canonical
  Git URLs.

## Canonicalization

- wgpu-graft commit `020b800` updates package metadata and repairs CI commands
  after the package rename to `grafting`.
- The CI minimal-feature command now enables `wgpu-29`, matching the crate's
  compile-time feature requirement.
- wgpu-scry commit `0af3b54` and wgpu-weld commit `481911e` update their
  manifests, docs, and locks to the canonical source at `020b800`.

## Verification

- `cargo metadata --format-version 1 --no-deps`
- `cargo test -p grafting`
- `cargo check -p grafting --no-default-features --features wgpu-29`
- `cargo check -p grafting --all-features`
- Both adapter metadata graphs contain exactly one `grafting` source and no
  old-owner source.
