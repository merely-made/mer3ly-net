# Merely project showcase

**Status:** M7 complete; accepted by the
[project showcase receipt](receipts/site/2026-07-30_m7_project_showcase.md)

**Authority:** This is the canonical plan for turning Mer3ly's repository
inventory into a public project showcase. It follows the completed
[live repository graph and organization migration
plan](2026-07-29_live_repos_graph_and_org_migration_plan.md) without reopening
its ownership or deployment decisions.

## Purpose

The repository map proves how Merely's public work fits together. M7 adds the
editorial layer that lets a visitor understand why an individual project
exists, see a truthful current state, and move between that project and its
neighbors.

The static semantic site remains the product. Images and the Mere-arranged
graph are progressive evidence, not requirements for reading it.

## Settled boundaries

- GitHub Pages remains the sole deployment origin. Cloudflare may proxy it.
- Cambium and Genet continue to generate ordinary static HTML.
- The repository graph and project pages consume the same committed repository
  and relation ids.
- `content/repositories.toml` owns repository identity, classification, status,
  summary, license, homepage, and GitHub slug.
- `content/relations.toml` owns typed inter-repository relationships.
- `content/showcases.toml` owns optional Mer3ly-specific editorial copy and
  visual attribution. It may reference only public repository ids.
- Every public repository receives a generated project profile. A screenshot is
  optional and must never be used to make a prototype look more complete than
  it is.
- The archived Merely design-system bundle is a visual and editorial reference,
  not a runtime dependency or an authority for unresolved product themes.

## Licensing

- Mer3ly source code is licensed under MPL-2.0.
- Original Mer3ly prose and site artwork are licensed under CC BY 4.0.
- Imported project screenshots retain the license of their source repository.
  Each imported image records its public source URL and a plain attribution.
- Merely names and marks are not licensed as trademarks by either grant.

The repository root must state these scopes without suggesting that Merely can
relicense third-party material shown inside a project screenshot.

## M7 authority

Each `[[showcase]]` record contains:

- `repository`: a public repository id;
- `order`: a unique positive display order;
- `headline`: a short plain-language product claim;
- `copy`: one paragraph grounded in the current repository;
- `image`: a normalized PNG path beneath `assets/showcase/`;
- `alt`: an exact description of the visible state;
- `caption`: a short product and state label;
- `source_url`: the public source-repository path for the imported screenshot;
- `source_license`: the source repository's declared license.

Validation rejects duplicate repositories or orders, unknown or non-public
repositories, empty editorial fields, non-PNG or escaping paths, missing image
files, mismatched source repository URLs, and license disagreement with the
repository authority.

## Pages

### Home

The home page keeps the community-radio field brief, then presents the selected
software projects as substantial visual entries rather than generic cards.
Each entry links to its local project profile. The copy must preserve the
current company framing: Merely LLC is the company; "Merely made" is at most a
tagline.

### Project profiles

Generate `/projects/<repository-id>/index.html` for every public repository.
Each profile includes:

- repository name, class, status, and summary;
- optional showcase image, headline, copy, caption, and attribution;
- license, language, topics, and validated metadata date;
- incoming and outgoing typed relationships;
- links to related local profiles, the public repository, and any distinct
  project homepage;
- a route back to the complete repository map.

Repository cards and the graph's explicit open action link to these profiles.
The complete repository index remains available without JavaScript, WebAssembly,
WebGPU, or images.

## Visual import and privacy

- M7 begins with the already reviewed README images for Mere, Genet/Pelt,
  Turnstone, Woodshed, and Isometry.
- Imported PNGs are normalized by a checked script that validates the PNG
  structure and removes text, EXIF, timestamp, and other unapproved ancillary
  chunks.
- The artifact validator scans every generated text asset and enforces the
  exact generated file set, including project pages and showcase images.
- The validator must not echo any discovered secret or personal value.
- No source path, workstation name, user profile, private hostname, network
  address, device identifier, or unapproved contact address may enter the
  artifact or a receipt.

## New game repositories

Mesocosm and Paredros were read on 2026-07-30. Both are pre-implementation
name-reservation repositories with design records and `MIT OR Apache-2.0`
scaffolds. They form first-person and second-person vessels beside Isometry's
third-person game, sharing a world substrate, lineage model, and trust plane
without sharing an engine or schedule.

Neither repository is currently available under `merely-made` or `mark-ik`, and
the local checkouts have no configured remote. They therefore remain deferred
showcase candidates. They enter public authority only after a public remote
exists and its description, topics, license, and intended relationships can be
verified. M7 does not manufacture links or graph edges for them.

## Milestones

### M7A: licensing and authority

- Add the repository license scope.
- Add and validate `content/showcases.toml`.
- Normalize and commit the five approved project images.

**Done when:** authority validation accepts every showcase record, image source,
license, and normalized asset, and rejects representative invalid records.

### M7B: project profiles

- Generate one profile per public repository.
- Project incoming and outgoing relations from the existing relation authority.
- Add local profile links to repository cards and the graph open action.

**Done when:** every public repository id appears once in `/repos/`, once as a
graph node, and once as a generated project directory; every relation appears
twice in repository text, twice across the corresponding project profiles, and
once in graph data.

### M7C: home showcase

- Replace the generic project grid with authority-backed visual entries.
- Preserve the radio field brief and the site's no-script reading order.
- Add responsive image, profile, and relation treatments using the current
  Mer3ly visual language.

**Done when:** desktop and narrow views have no clipping, overlap, unreadable
text, stretched screenshots, or empty visual frames; keyboard focus and reduced
motion remain explicit.

### M7D: artifact and deployment acceptance

- Expand exact artifact validation to project pages and normalized images.
- Run Rust formatting, tests, clippy, authority validation, metadata
  validation, JavaScript syntax checks, and local headed browser smoke.
- Scan the staged diff and artifact for sensitive information.
- Commit in logical batches, push, deploy through the existing Pages workflow,
  and compare the public routes with the accepted artifact.

**Done when:** the checked Pages artifact and public Cloudflare edge agree for
the home, repository map, radio page, and representative visual and text-only
project profiles.

## Stop rules

- Do not add Mesocosm or Paredros to public authority before their public
  repositories exist.
- Do not scrape README prose or images during a Pages build. Public builds use
  only committed, reviewed authority and assets.
- Do not add a content management framework, client-side router, image CDN, or
  second deployment origin.
- Do not begin a reusable cross-product design-system implementation from this
  website slice. Promote patterns only after a second real product consumes
  them.
