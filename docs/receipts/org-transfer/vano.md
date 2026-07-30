# Vano organization transfer

**Transferred:** 2026-07-30
**Former slug:** `mark-ik/vano`
**Canonical slug:** `merely-made/vano`
**Upstream:** `trynova/nova`
**Default branch:** `main`
**Transferred head:** `1816f328fab7151a06e999d4353022fabd9b7d28`

## Pre-transfer gate

- Public MPL-2.0 GitHub fork with reviewed Nova provenance.
- Active downstream tree and fork-only history passed the redacted sensitive
  scan. Exact machine profile, machine name, configured contact address, and
  first-party private endpoint counts were zero.
- Repository settings contained zero secrets, variables, environments, deploy
  keys, webhooks, Pages sites, releases, rulesets, or branch protection.
- Actions was enabled for all actions with two workflows.
- `merely-made/vano` had no name or fork-network collision.
- `genet-embedder` contained the maintained integration line and cleanly
  fast-forwarded the old `main`.

## Project identity

Commit `1816f328`:

- names Vano as Merely Made's data-oriented JavaScript engine for Genet;
- preserves the Nova upstream attribution and project links;
- points workspace and package repository metadata at
  `https://github.com/merely-made/vano`;
- updates downstream Serval-era comments to current Genet terminology.

`cargo metadata --locked --no-deps` and
`cargo check -p nova_vm --locked --offline` passed. Workspace-wide
`cargo fmt --check` still reports pre-existing formatting differences in
unrelated engine files; this transfer did not reformat them.

## Post-transfer verification

- GitHub canonical slug: `merely-made/vano`.
- Fork parent and source: `trynova/nova`.
- Local origin: `https://github.com/merely-made/vano.git`.
- Local `main`, `origin/main`, old Git URL, and new Git URL resolve
  `1816f328fab7151a06e999d4353022fabd9b7d28`.
- `https://github.com/mark-ik/vano` redirects with a successful response to
  `https://github.com/merely-made/vano`.
- The older `mark-ik/nova` alias also reaches the canonical repository and
  resolves the same `main` head.
- Post-transfer Actions remains enabled with two workflows and zero secrets,
  variables, environments, deploy keys, or webhooks.
- Pages and releases remain absent.
- Repository description:
  `Data-oriented JavaScript engine for Genet, maintained as a Nova fork.`

Genet commit `5a8d2734` canonicalizes its tracked Vano references. Its ignored
local lock preserves Vano commit `22b42989`, and a clean-path locked offline
check compiled the source from `merely-made/vano`.
