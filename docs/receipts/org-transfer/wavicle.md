# Wavicle organization transfer

Verified at `2026-07-29T21:35:26Z`.

## Transfer

- Source: `mark-ik/wavicle`
- Target: `merely-made/wavicle`
- Pre-transfer head: `a5b5b7c12a9c75cc04c2c795c6ce627fa79450ce`
- Post-transfer head: `37fea3cda8dbbe3830aafc80368775f633010e6b`
- Default branch: `main`
- Visibility: public
- Local origin: `https://github.com/merely-made/wavicle.git`

The pre-transfer audit found one Actions workflow and no Pages site, package,
repository ruleset, secret, variable, deploy key, webhook, or release.

## Redirects

- The former web URL redirects to `https://github.com/merely-made/wavicle`.
- The former API slug resolves as `merely-made/wavicle`.
- `git ls-remote` returns the same `main` head through the former and canonical
  Git URLs.

## Canonicalization

- Wavicle commit `37fea3c` updates the package repository metadata.
- Hocket commit `4641fb3` updates its manifest, documentation, and clean lock.
- Hocket's lock contains one Wavicle source, under `merely-made`.

## Verification

- `cargo metadata --format-version 1 --no-deps`
- `cargo test --features encode`
- Three unit tests and fourteen conformance tests pass.
- Hocket model tests and `cargo check -p hocket-genet --locked` pass against
  the clean canonical lock.
