# Merely authority reconciliation

**Status:** M9 in progress

**Authority:** This is the canonical plan for reconciling the public Mer3ly
catalog with the live `merely-made` organization after the accepted
[M8 discovery and sharing](receipts/site/2026-07-30_m8_discovery_and_sharing.md)
milestone.

## Purpose

M8 accepted a discoverable 16-repository site. Three original, public Merely
repositories now meet the same publication boundary: Mesocosm, Paredros, and
Tulpa. The migration ledger also retains two stale unresolved product names
that have since been resolved by live project documentation.

M9 makes the authority truthful again without manufacturing architecture for
name reservations. It also separates reproducible source inputs from generated
deployment output so the checked public artifact has one clear owner.

## Authority decisions

- Mesocosm and Paredros are public product prototypes. Their repositories hold
  name reservations and design records rather than implemented games.
- Tulpa is a public foundation research record. It reserves the memorial and
  legend layer but is not yet an implemented Mere organ.
- All three receive text-first project profiles and reduced public GitHub
  metadata.
- None receives a graph relation in M9. A shared design record, intended
  integration, or sibling status is not an implemented dependency.
- Merecat is no longer unresolved: current Mere documentation identifies
  Turnstone as the browser host formerly called Merecat.
- Strophe is no longer unresolved: Hocket's documentation records Strophe as
  its former name.
- GitHub's single detected license is advisory. The committed manifests,
  READMEs, and paired license files establish `MIT OR Apache-2.0` for all three
  repositories.

## Generated artifact boundary

The committed inputs are Rust source, templates, curated repository and
relation authority, the baseline reduced metadata cache, showcase records,
site assets, and the pinned graph runtime source.

`html/` is a local generated preview and is ignored. Production builds refresh
reduced public metadata, rebuild the Wasm client on the deployment host, and
write the exact checked artifact to `.tmp/pages-artifact`. GitHub Pages deploys
that artifact and remains the sole origin.

The workflow gains a path-filtered `main` push trigger. Changes to site source,
authority, assets, runtime inputs, or the workflow publish automatically.
Documentation-only changes do not rebuild production.

## Implementation seams

- `content/repositories.toml` owns the three new public records.
- `ops/org-migration.toml` owns publication evidence and removes the two stale
  unresolved aliases.
- `content/github-metadata.json` carries one reduced record per public
  repository after an authenticated refresh.
- `.gitignore` and `README.md` state the generated-output boundary.
- `.github/workflows/pages.yml` owns the path-filtered deployment trigger and
  M9 acceptance artifact.
- `tests/m8_discovery.rs` and `scripts/smoke-site.mjs` enforce the new exact
  public counts.

## Milestones

### M9A: reconcile public authority

- Add Mesocosm, Paredros, and Tulpa.
- Remove the Merecat and Strophe unresolved entries.
- Refresh reduced public metadata after assigning bounded GitHub topics.
- Keep the relation manifest unchanged.

**Done when:** authority validates at 19 repositories, 25 relations, 29
migration records, and zero unresolved products; metadata has 19 matching
records; every new profile is text-first.

### M9B: establish artifact truth

- Ignore local `html/` output and remove it from the tracked source tree.
- Describe the baseline metadata and production refresh separately.
- Add the path-filtered push deployment trigger.
- Use milestone-neutral local smoke output and M9 workflow receipt names.

**Done when:** a clean checkout can build and validate the exact temporary
artifact without a committed generated site tree.

### M9C: accept and publish

- Run formatting, tests, clippy, authority validation, metadata validation,
  exact-artifact validation, and browser smoke.
- Scan the new repositories, staged source, and generated public output for
  secrets, identifying local paths, machine names, configured contact
  addresses, and private network addresses.
- Commit in logical batches, push `main`, and accept the resulting Pages
  deployment against its checked artifact.

**Done when:** the public site exposes 19 repository cards, 19 project
profiles, 25 graph edges, 50 semantic relationship projections, and 22
canonical sitemap URLs; the edge matches the accepted artifact byte for byte.

## Stop rules

- Do not add conceptual or sibling graph edges.
- Do not invent screenshots for name-reservation repositories.
- Do not publish private metadata, repository administration state, analytics,
  crawler credentials, or broad cross-repository credentials.
- Do not add a second deployment origin, CMS, client router, service worker, or
  design-system abstraction.
