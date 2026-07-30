# Mer3ly

The public site for [Merely](https://mer3ly.net/). It is a small static Rust
build: Cambium views construct Genet `ScriptedDom` documents, which are
serialized into ordinary HTML. The repository page progressively adds a small
Mere-arranged WebGPU graph over the same committed public records.

The complete site remains readable without JavaScript, WebAssembly, or WebGPU.
Source CSS, graph runtime, and social preview live under `assets/`; generated
deployment files live under `html/`.

## Build

Refresh the committed public GitHub metadata cache when authenticated `gh`
access is available:

```powershell
.\scripts\refresh-public-metadata.ps1
```

The refresh validates a complete temporary snapshot before replacing the
cache. A failed refresh leaves the last valid public snapshot in place.

When changing the repository graph client, rebuild its committed Wasm runtime:

```powershell
.\scripts\build-repo-graph.ps1
```

The script compiles the nested client crate against a pinned Mere revision,
runs `wasm-bindgen`, copies the deployable module into `assets/`, and removes
its temporary Cargo target.

Generate the static home, community-radio, and repository pages with:

```powershell
cargo run --locked --bin site
```

Write to a different directory with:

```powershell
cargo run --locked --bin site -- --output path/to/output
```

## Verify

```powershell
cargo test --locked
cargo test --manifest-path crates/repo-graph/Cargo.toml --locked
cargo clippy --locked --all-targets -- -D warnings
cargo run --locked --bin authority -- validate
cargo run --locked --bin authority -- validate-metadata
npm ci --ignore-scripts
npx playwright install chromium
npm run smoke
```

The browser smoke serves committed `html/` locally and checks the home,
community-radio, desktop, mobile, reduced-motion, and WebGPU-fallback paths.
Set `MER3LY_SITE_DIR` to check another generated site directory.

`authority validate-artifact` accepts a Pages artifact root after the
repository root. It enforces the exact public file set, public authority and
graph counts, displayed metadata timestamp, Wasm header, reduced GitHub links,
and the absence of secrets, personal data patterns, local paths, and private
network addresses. It emits a JSON receipt with SHA-256 hashes:

```powershell
cargo run --locked --bin authority -- validate-artifact . .tmp/pages-artifact
```

## Deployment

[`pages.yml`](.github/workflows/pages.yml) refreshes the reduced public
metadata cache, rebuilds and validates the exact static artifact, runs a
headed Chromium smoke under a virtual display, and deploys that artifact to
GitHub Pages. It runs on manual dispatch and a daily schedule. The build has
read-only repository permission; only the separate deployment job receives
the Pages and identity grants. The graph runtime is built twice and must have
identical hashes on the deployment host before either output can be published.

## Plans

- [Live repository graph and Merely Made organization migration](docs/2026-07-29_live_repos_graph_and_org_migration_plan.md)
- [Site acceptance receipts](docs/receipts/site/README.md)
