# M5 live Mere repository graph

**Date:** 2026-07-30 UTC

**Implementation commit:** `80351f6`

**Pages run:** `30526690649`

**Result:** model, static fallback, native and Wasm builds, privacy,
responsive-browser, keyboard, pointer, reduced-motion, and failure-path checks
and public deployment accepted. M6 was not started.

## What changed

- `crates/repo-graph` is a thin Wasm projection over Mere's pinned
  `arrangements` crate at revision
  `d18cfdf7c2695dbfa271a1c263c09b0d2248ab89`.
- Mere's radial layout keeps `mere` as the focus. Records unreachable from
  that focus use a stable side lane so a long dependency chain does not
  collapse the connected family into a knot.
- `assets/repo-graph.js` renders edges and node discs through WebGPU. Semantic
  HTML buttons provide visible labels, focus, selection, keyboard traversal,
  and links to the complete repository entries below.
- The renderer draws only after a change, does not run a continuous animation
  loop, and skips frames while the document is hidden.
- The authored loader is an optional module at the end of the already-complete
  page. WebGPU, Wasm, and initialization failures restore the visible static
  message and leave all repository cards present.

The client does not load Graphshell application state, browser history,
Personae identity, or a resident Mere host.

## Shared authority

Both projections consume schema `mer3ly.repo-graph/v1`.

| Record | Semantic page | Live graph |
| --- | ---: | ---: |
| Public repository ids | 16 | 16 |
| Typed relationship ids | 25 | 25 |

The Rust tests compare the exact id sets and verify that every graph edge has
known endpoints. Relation kind and provenance remain in the graph payload and
have matching text in the semantic index and legend.

## Runtime artifacts

| Artifact | Bytes |
| --- | ---: |
| `repo-graph.js` | 22,564 |
| `mer3ly_repo_graph.js` | 8,014 |
| `mer3ly_repo_graph_bg.wasm` | 150,741 |
| Total optional graph runtime | 181,319 |

The Wasm build remaps the local profile root and strips symbols before the
asset is committed.

## Verification

These commands passed:

```powershell
.\scripts\build-repo-graph.ps1
cargo fmt --all -- --check
cargo fmt --manifest-path crates/repo-graph/Cargo.toml -- --check
cargo test --locked
cargo test --manifest-path crates/repo-graph/Cargo.toml --locked
cargo clippy --locked --all-targets -- -D warnings
cargo clippy --manifest-path crates/repo-graph/Cargo.toml --locked --all-targets -- -D warnings
cargo run --locked --bin authority -- validate
cargo run --locked --bin authority -- validate-metadata
node --check assets/repo-graph.js
node --check assets/mer3ly_repo_graph.js
```

The root suite ran 14 tests. The nested graph crate ran four more tests for
identity, deterministic layout, stable unreachable placement, and endpoint
rejection.

## Headed browser review

The ordinary 1440 by 900 path reported:

| State | Result |
| --- | --- |
| Graph state | `ready` |
| Engine | `mere-arrangements/radial+unreachable-lane` |
| Nodes | 16 |
| Edges | 25 |
| Horizontal overflow | 0 pixels |
| Browser diagnostics | none |

At 420 by 900, the graph remained ready with 16 nodes, no horizontal
overflow, a 500-pixel canvas, hidden idle labels, and a one-column legend. A
375 by 812 fallback path also had zero horizontal overflow.

Keyboard focus moved from Mere to Retinue with the right arrow. Retinue became
the selected node and the live region announced:

```text
Retinue selected. 0 outgoing and 1 incoming relationships.
```

Enter focused `repo-retinue` in the semantic index. Zoom-in moved Mere's
screen x-coordinate from 668.267 to 706.120 pixels, pan-right moved it to
670.120 pixels, and fit restored 668.267 pixels. A headed pointer drag moved
the same node from 668.267 to 718.267 pixels.

The reduced-motion query receipt initialized normally with
`data-reduced-motion="true"`. Opening a repository uses instant rather than
smooth scrolling in that path.

Forced failure receipts:

| Query | Graph state | Interface | Repository cards |
| --- | --- | --- | ---: |
| `?graph=no-webgpu` | `unavailable` | hidden | 16 |
| `?graph=no-wasm` | `unavailable` | hidden | 16 |
| `?graph=init-failure` | `unavailable` | hidden | 16 |

## Captures

- [Desktop live graph](m5/repository-graph-desktop.png)
- [Mobile live graph](m5/repository-graph-mobile.png)
- [WebGPU-unavailable fallback](m5/repository-graph-fallback.png)

## Privacy and transport

The dirty additions were scanned for the local profile path and account name,
machine name, loopback and private-network addresses, email addresses,
credential shapes, and private-key blocks. The scan reported zero accidental
findings. A first Wasm build did contain local Rust source paths; it was
rejected, rebuilt with profile-root remapping and symbol stripping, and then
covered by the committed binary privacy test.

Cloudflare currently returns:

| Request | Result |
| --- | --- |
| `https://mer3ly.net/` | 200 with `Strict-Transport-Security: max-age=2592000` |
| `http://mer3ly.net/repos/` | 301 to `https://mer3ly.net/repos/` |

## Deployment

GitHub Pages deployed acceptance head `f5b3a54` successfully in run
`30526690649`. The Cloudflare-fronted public site then reported:

| Check | Result |
| --- | --- |
| `/repos/` static authority | 16 repository cards and 50 relation text projections |
| Live graph | `ready`, 16 nodes, 25 edges |
| Layout engine | `mere-arrangements/radial+unreachable-lane` |
| Desktop overflow | 0 pixels at 1440 by 900 |
| Browser diagnostics | none |
| Forced WebGPU failure | `unavailable`, interface hidden, 16 cards retained |
| Graph JavaScript MIME | `text/javascript` |
| Graph Wasm MIME | `application/wasm` |
| HSTS | `max-age=2592000` |
| Plain HTTP `/repos/` | 301 to the matching HTTPS URL |

## Stop

M5 implementation and local acceptance are complete. Stop before M6 metadata
and deployment automation.
