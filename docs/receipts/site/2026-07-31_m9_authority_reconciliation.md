# M9 authority reconciliation

**Accepted:** 2026-07-31

**Source:** `82bcd0d308f436b3bbc0cb51eb1c89979476e796`

**Deployment run:** [30606708420](https://github.com/merely-made/mer3ly-net/actions/runs/30606708420)

## Result

The public Mer3ly authority now contains Mesocosm, Paredros, and Tulpa as
truthful text-first profiles. Merecat and Strophe are removed from the
unresolved ledger because current project documentation identifies them as
former names for Turnstone and Hocket.

The relation manifest remains unchanged. The three name-reservation
repositories have no implementation-backed dependency or curated architecture
edge.

## Authority

The accepted authority contains:

- 19 public repositories;
- 25 relationship edges and 50 semantic relationship projections;
- 29 migration records;
- zero unresolved products;
- 19 reduced public GitHub metadata records;
- 19 project profiles;
- 22 canonical sitemap URLs.

Mesocosm and Paredros are product prototypes. Tulpa is a foundation research
record. Their committed manifests, READMEs, and paired license files establish
`MIT OR Apache-2.0`; GitHub's single detected Apache license is not treated as
the complete licensing authority.

The repositories received bounded public topics:

- Mesocosm: `ecosystem-simulation`, `evolution`, `game-design`, `rust`,
  `voxel`;
- Paredros: `action-rpg`, `companions`, `game-design`, `rust`,
  `second-person`;
- Tulpa: `data-structures`, `history`, `memory`, `provenance`, `rust`.

## Checked artifact

The successful workflow built and deployed a 36-file Pages artifact totaling
1,734,179 bytes. The Pages upload was artifact `8783982162`; headed and static
acceptance evidence was artifact `8783981524`.

The artifact validator accepted:

- 19 graph nodes and 25 graph edges;
- 19 project profiles and 50 project relationship projections;
- 5 approved showcase images;
- 22 unique canonical sitemap URLs;
- 19 project social-preview records;
- 19 project structured-data records;
- refreshed public metadata generated at `2026-07-31T05:28:41Z`, with
  SHA-256
  `e406aac488010af4a01bae903337b4d44d444f1112a01154ea42800fa53a9c7a`.

`html/` is no longer tracked. It remains an ignored local preview directory.
Production refreshed metadata, rebuilt the deterministic Linux Wasm client,
and deployed the exact checked `.tmp/pages-artifact` output.

## Browser acceptance

Headed Chromium `151.0.7922.34` accepted:

- all representative routes at HTTP 200 with one `h1`, no browser errors, and
  no horizontal overflow;
- the Mere visual profile and the new Mesocosm text-first profile;
- 5 decoded showcase images at desktop and mobile widths;
- 19 repository records, 25 graph edges, and 50 semantic relationship
  projections at desktop and mobile widths;
- reduced-motion media handling;
- the forced no-WebGPU fallback.

The Linux software WebGPU device did not remain available during M9. The first
push run exposed a race between a ready graph and the supported device-loss
fallback. The smoke now addresses nodes by stable authority ID and accepts the
fallback only when the graph has actually changed to `unavailable`; unrelated
interaction failures still fail the run. The successful receipt records the
fallback path, so M9 does not claim fresh headed WebGPU interaction proof.

## Public edge

After deployment, browser-shaped requests fetched every deployable artifact
file from `https://mer3ly.net`. All 35 public files matched the accepted Pages
artifact byte for byte. `CNAME` is deployment metadata and is not counted as a
public route.

All 22 HTML documents were free of Cloudflare Web Analytics beacon injection.
The edge returned `Server: cloudflare` and
`Strict-Transport-Security: max-age=2592000`.

## Privacy and source checks

Mesocosm, Paredros, and Tulpa were clean and synchronized before publication.
Gitleaks `8.30.1` found zero secrets in each current tree and full history.
Exact scans found no tracked user-profile path, machine name, configured
contact address, private network address, or old personal-owner link.

The changed Mer3ly source received a separate zero-finding Gitleaks scan. The
exact-artifact validator checked every generated public text asset. The only
private-network source match remains the validator's synthetic rejection
fixture.

No sensitive output required a new ignore rule. The new `/html/` ignore records
generated-output ownership, not sensitive information.

## Verification

Local verification passed:

```powershell
cargo fmt --all -- --check
cargo fmt --manifest-path crates/repo-graph/Cargo.toml -- --check
cargo test --locked
cargo test --manifest-path crates/repo-graph/Cargo.toml --locked
cargo clippy --locked --all-targets -- -D warnings
cargo clippy --manifest-path crates/repo-graph/Cargo.toml --locked --all-targets -- -D warnings
cargo run --locked --bin authority -- validate
cargo run --locked --bin authority -- validate-metadata
cargo run --locked --bin authority -- validate-artifact . .tmp/m9-artifact
node --check scripts/smoke-site.mjs
npm run smoke
```

The root suite passed 28 tests. The repository-graph crate passed 4 more. The
successful push workflow repeated authority, metadata, source, reproducibility,
artifact, headed browser, and deployment checks on Linux.

## Acceptance

M9 is accepted. The public edge has 19 repository profiles and 22 sitemap
documents, while the evidence-backed graph remains at 25 edges. The committed
source tree no longer presents generated `html/` as deployment authority.
