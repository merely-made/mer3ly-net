# Mere organization transfer

Verified at `2026-07-29T23:24:55Z`.

## Transfer

- Source: `mark-ik/mere`
- Target: `merely-made/mere`
- Pre-transfer head: `48e277d42ac90baf2f98281f6cbc879eba099bf4`
- Post-transfer head: `d5af0618cb30077af3f1df11b65dc6e7e0da2bd0`
- Default branch: `main`
- Visibility: public
- Local origin: `https://github.com/merely-made/mere.git`

The pre-transfer audit found no Actions workflow, Pages site, package,
repository ruleset, secret, variable, deploy key, webhook, or release.
Post-transfer Actions remain enabled for all actions, with zero repository
secrets and variables.

## Redirects

- The former web URL redirects to `https://github.com/merely-made/mere`.
- The former API slug resolves as `merely-made/mere`.
- `git ls-remote` returns the same `main` head through the former and canonical
  Git URLs.

## Canonicalization

- Mere commit `d5af0618cb3` updates 43 tracked package metadata, dependency,
  patch-table, source, and documentation files.
- Hocket commit `d89c862`, Isometry commit `885d7dd`, Woodshed commit
  `99eba4a`, and Turnstone commit `f019b23` canonicalize their Mere edges and
  locks.
- Tracked repositories outside the historical Mer3ly receipts contain zero
  `mark-ik/mere` references.

## Verification

- Clean `cargo metadata --locked --offline` resolves Genet at `2955d41c` and
  Retinue at `7439a79d`.
- `cargo check -p content-contract -p mere-canvas -p mere-gloss --locked
  --offline`
- Clean product locks resolve one Mere source at `d5af0618`.
- Hocket, Isometry, Woodshed, and Turnstone passed their focused locked
  verification walls against the post-transfer Mere head.
