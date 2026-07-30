# M7 project showcase

**Date:** 2026-07-30 UTC

**Acceptance commit:** `c6cdb75ca52d57d3f0e4b8b033520eeb776fdea1`

**Pages run:** `30570463227`

**Result:** licensing, bounded showcase authority, normalized project images,
semantic project profiles, responsive presentation, exact-artifact validation,
headed smoke, deployment, privacy checks, and public-edge comparison accepted.

## Public shape

The generated site now contains a profile for every public repository. The
home page presents five current visual entries: Mere through Graphshell,
Genet through Pelt, Turnstone, Woodshed, and Isometry. Projects without an
approved current image use an explicit text-first profile rather than a
placeholder or an overstated visual.

Every profile projects the same committed repository and relation authority as
the semantic repository index and Mere-arranged graph. Repository cards and
the graph's open action lead to local profiles. The complete site remains
readable without JavaScript, WebAssembly, WebGPU, or images.

Mesocosm and Paredros remain deferred. Their local design records were read,
but neither project had a public remote at acceptance, so M7 did not create
public links, metadata, or graph edges for them.

## Checked artifact

Run `30570463227` built and deployed source
`c6cdb75ca52d57d3f0e4b8b033520eeb776fdea1`. The Pages upload was artifact
`8770775373`; acceptance evidence was artifact `8770774245`.

The validator accepted:

| Record | Result |
| --- | ---: |
| Files | 30 |
| Total bytes | 1,674,706 |
| Public repositories | 16 |
| Relation text projections | 50 |
| Graph nodes | 16 |
| Graph edges | 25 |
| Project profiles | 16 |
| Project relation projections | 50 |
| Showcase images | 5 |
| Metadata refresh | 2026-07-30 18:31 UTC |

The graph runtime was built twice on the deployment host and produced
identical outputs. The deployed Wasm SHA-256 was
`041fe97e0c9265aef07740b10370a6aa2031f2bedfbcfbb833684d7f5c919d9c`.

## Exact public match

After Cloudflare Web Analytics automatic injection was disabled, browser
navigation responses contained no Cloudflare beacon. Twenty-nine deployable
files matched the accepted Pages artifact byte for byte. `CNAME` is deployment
metadata and was not requested as a public route.

Representative routes returned:

| Route | Status | SHA-256 |
| --- | ---: | --- |
| `/` | 200 | `74ac356a1eee1a6da99830206d1510ea9e1cae2c90b84500acb6b622df5853bb` |
| `/repos/` | 200 | `1520784f54c6637918d838d881e4a2f46b5637386d66d9f8d8bf84a4fa72d097` |
| `/radio.html` | 200 | `279a4f64b57850272e50f938ca27127147350812a5276d16c1dbdd8f380867f6` |
| `/projects/mere/` | 200 | `e0c7936ee4b7da61d4be6f4bbf6d2020c90b2a720c7e0cad5a000cef87bde623` |
| `/projects/retinue/` | 200 | `58ad6b759186a45a08e3180d93ccb43f14fd2502d956edebe560fd0063612c14` |

The public HTTPS response retained
`Strict-Transport-Security: max-age=2592000`.

## Browser evidence

The workflow's headed Chromium smoke accepted:

| Surface | Result |
| --- | --- |
| Home showcase, desktop | 5 cards, 5 decoded images, 0 pixels overflow |
| Home showcase, mobile | 5 cards, 0 pixels overflow |
| Mere visual profile | 1 approved image, 0 pixels overflow |
| Retinue text-first profile | 0 images, 0 pixels overflow |
| Repository authority | 16 repositories, 50 relation text projections |
| Graph authority | 16 nodes, 25 edges |
| Reduced motion | acknowledged by the graph client |
| Forced WebGPU fallback | complete semantic repository index retained |

The hosted runner did not expose WebGPU for its ordinary desktop and mobile
passes, so those receipts accepted the unavailable state and complete static
fallback. A separate headed local browser pass initialized the graph, selected
Retinue, opened `/projects/retinue/`, and reported no console warnings or
horizontal overflow.

## Licensing and privacy

Mer3ly source is licensed under MPL-2.0. Original Mer3ly prose and site artwork
are licensed under CC BY 4.0. Imported screenshots retain the declared license
of their source repository and carry public source attribution.

The five showcase PNGs were structurally normalized. Re-running the importer
produced byte-identical files, proving that no text, EXIF, timestamp, or other
unapproved ancillary chunk remained.

The staged and generated text scans found no machine paths, private network
addresses, credentials, local user identifiers, or unapproved contact
addresses. The validator's rejection fixtures are synthetic and do not echo
the values they detect. Build targets, temporary receipts, browser captures,
and downloaded deployment evidence remain ignored.

## Verification

Local checks passed:

```powershell
cargo fmt --all -- --check
cargo fmt --manifest-path crates/repo-graph/Cargo.toml -- --check
cargo test --locked
cargo test --manifest-path crates/repo-graph/Cargo.toml --locked
cargo clippy --locked --all-targets -- -D warnings
cargo clippy --manifest-path crates/repo-graph/Cargo.toml --locked --all-targets -- -D warnings
cargo run --locked --bin authority -- validate
cargo run --locked --bin authority -- validate-metadata
cargo run --locked --bin authority -- validate-artifact . .tmp/m7-final-artifact
npm run smoke
node --check assets/repo-graph.js
node --check scripts/smoke-site.mjs
node --check scripts/import-showcase-png.mjs
```

The root suite passed 25 tests across the library and M3-M7 integration
boundaries. The nested graph crate passed four tests. The committed `html/`
tree matched a fresh generated artifact across all 30 files.

## Stop

M7 is accepted. A new repository enters the public map only after its public
remote, metadata, license, and intended relationships can be verified.
