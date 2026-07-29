# Smolweb organization transfer

Verified at `2026-07-29T21:35:26Z`.

## Transfer

- Source: `mark-ik/smolweb`
- Target: `merely-made/smolweb`
- Pre-transfer head: `bcc70c75ca6a350240ebeb3b15c3ca6ec6dbc83c`
- Post-transfer head: `95d110c3a76c6cd3f06c1c4c39f56ca3804d8273`
- Default branch: `main`
- Visibility: public
- Local origin: `https://github.com/merely-made/smolweb.git`

The pre-transfer audit found one Actions workflow and no Pages site, package,
repository ruleset, secret, variable, deploy key, webhook, or release.

## Redirects

- The former web URL redirects to `https://github.com/merely-made/smolweb`.
- The former API slug resolves as `merely-made/smolweb`.
- `git ls-remote` returns the same `main` head through the former and canonical
  Git URLs.

## Canonicalization

- Smolweb commit `95d110c` updates all package repository metadata.
- Genet commit `4ca5b45f0aa` and Mere commit `48e277d4` update their public
  dependency documentation.
- The tracked source scan contains no active `mark-ik/smolweb` reference.

## Verification

- `cargo metadata --format-version 1 --no-deps`
- `cargo test`
- Smolweb unit and doctest suites pass.
