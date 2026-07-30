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
```

## Plans

- [Live repository graph and Merely Made organization migration](docs/2026-07-29_live_repos_graph_and_org_migration_plan.md)
- [Site acceptance receipts](docs/receipts/site/README.md)
