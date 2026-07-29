# Genet organization transfer

Verified at `2026-07-29T23:24:55Z`.

## Transfer

- Source: `mark-ik/genet`
- Target: `merely-made/genet`
- Pre-transfer head: `4ca5b45f0aa79b9ae6d1e2f4592cdea64a89f58e`
- Post-transfer head: `2955d41c6c981f650f8c2274e8f5cba9aea08a96`
- Default branch: `main`
- Visibility: public
- Local origin: `https://github.com/merely-made/genet.git`
- Fork ancestry: `servo/servo`

The pre-transfer audit found three Actions workflows and one empty,
unprotected environment. It found no Pages site, package, repository ruleset,
secret, variable, deploy key, webhook, or release. Post-transfer Actions
remain enabled for all actions, with zero repository secrets and variables.

## Redirects

- The former web URL redirects to `https://github.com/merely-made/genet`.
- The former API slug resolves as `merely-made/genet`.
- `git ls-remote` returns the same `main` head through the former and canonical
  Git URLs.

## Canonicalization

- Genet commit `2955d41c6c9` updates 51 tracked package metadata and
  documentation files.
- Mere commit `d5af0618cb3` canonicalizes its Genet dependencies.
- Hocket commit `d89c862`, Isometry commit `885d7dd`, Woodshed commit
  `99eba4a`, and Turnstone commit `f019b23` canonicalize their Genet edges and
  locks. Smolweb commit `8a7eac7` and wgpu-scry commit `ec6b456` update
  documentation links.
- Tracked repositories outside the historical Mer3ly receipts contain zero
  `mark-ik/genet` references.

## Verification

- `cargo metadata --format-version 1 --no-deps`
- `cargo check -p sprigging -p scrying-engine`
- Clean locked metadata resolves one Genet source at `2955d41c`.
- Hocket: 38 model tests and `cargo check -p hocket-genet`
- Isometry: workspace check with all features and all targets
- Woodshed: 167 tests, 4 doctests, and `cargo check -p woodshed-genet`
- Turnstone: 158 library tests passed; 4 tests remained explicitly ignored
