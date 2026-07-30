# Turnstone organization transfer

Verified at `2026-07-30T00:22:56Z`.

## Transfer

- Source: `mark-ik/turnstone`
- Target: `merely-made/turnstone`
- Pre-transfer head: `f019b23b960d48258861df0919d63357a810aaba`
- Post-transfer head: `ad017c97005f3ec1341e78fde11d31e620924506`
- Default branch: `main`
- Visibility: public
- Local origin: `https://github.com/merely-made/turnstone.git`

The pre-transfer audit found no Actions workflow, Pages site, package,
repository ruleset, secret, variable, deploy key, webhook, or release.
Post-transfer Actions remain enabled for all actions, with zero repository
secrets and variables.

## Redirects

- The former web URL redirects to `https://github.com/merely-made/turnstone`.
- The former API slug resolves as `merely-made/turnstone`.
- `git ls-remote` returns the same `main` head through the former and canonical
  Git URLs.

## Canonicalization

- Turnstone commit `ad017c97005` updates its package repository metadata.
- Mere commit `d18cfdf7c26` canonicalizes its public Turnstone links.
- Tracked repositories outside the historical Mer3ly receipts contain zero
  `mark-ik/turnstone` references.

## Verification

- `cargo metadata --format-version 1 --no-deps`
- Turnstone library tests: 158 passed; 4 remained explicitly ignored
