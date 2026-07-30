# M6 automated metadata and Pages deployment

**Date:** 2026-07-30 UTC

**Acceptance commit:** `248aecc90f7c8f09212135cdaa50aef7e01c1e60`

**Pages run:** `30557137565`

**Result:** reproducible build, bounded permissions, exact-artifact validation,
headed smoke, public-origin cutover, HTTPS, HSTS, and live graph checks
accepted. The migration and live repository graph plan is complete.

## Deployment boundary

GitHub Actions is the site build and deployment authority. The workflow runs
on manual dispatch and at `17 9 * * *` UTC. It does not deploy on ordinary
pushes.

The build job has read-only repository and Pages access. The separate deploy
job receives only `pages: write` and `id-token: write`. Deployment depends on
the complete build job, so authority, metadata, Rust, graph, artifact, or
browser failures stop publication before the deploy grant is used.

Cloudflare proxies the GitHub Pages origin for DNS, TLS, HTTPS redirection, and
HSTS. The former `mer3ly.net` Worker custom domain was detached, and its
`workers.dev` URL was disabled. The retained Worker project is not a public
site origin.

## Checked artifact

Run `30557137565` built and deployed source
`248aecc90f7c8f09212135cdaa50aef7e01c1e60`. The Pages upload was artifact
`8765482425`; acceptance evidence was artifact `8765481968`.

The site builder emitted a conventional artifact root:

```text
CNAME
index.html
mer3ly_repo_graph.js
mer3ly_repo_graph_bg.wasm
og.jpg
radio.html
repo-graph.js
repos/index.html
site.css
```

The validator accepted:

| Record | Result |
| --- | ---: |
| Files | 9 |
| Total bytes | 451,155 |
| Public repositories | 16 |
| Relation text projections | 50 |
| Graph nodes | 16 |
| Graph edges | 25 |
| Metadata refresh | 2026-07-30 15:36 UTC |

The graph runtime was built twice on the deployment host. Both builds produced
the same JavaScript and Wasm hashes. The deployed Wasm SHA-256 was
`041fe97e0c9265aef07740b10370a6aa2031f2bedfbcfbb833684d7f5c919d9c`.

## Exact public match

The public Cloudflare edge and direct GitHub Pages origin returned the same
checked documents:

| Route | Status | SHA-256 |
| --- | ---: | --- |
| `/` | 200 | `a529370d1b91d381a7305699b2ccf859da4574130a411b813ff6eae0f3b03cb1` |
| `/repos/` | 200 | `9596e9c8ac45342e978a52f021d13ca5014437b23a19675be5be37af5a688db0` |
| `/radio` | 200 | `48cc965086b77cd14b55fc2a1cf32667ade109ff1f0d6ae1f1894be8ef05841e` |

The proxied responses carried GitHub origin evidence, including
`x-github-request-id`, `via: 1.1 varnish`, and the Pages artifact's
`Last-Modified` value. Plain HTTP returned 301 to the matching HTTPS origin.
HTTPS returned `Strict-Transport-Security: max-age=2592000`.

## Browser evidence

The workflow's true-headed Chromium smoke passed under a virtual display. The
runner did not expose WebGPU, so its desktop, mobile, reduced-motion, and forced
fallback receipts accepted the complete static page with the graph marked
`unavailable`.

A headed public-browser check after the Cloudflare cutover supplied WebGPU and
reported:

| State | Result |
| --- | --- |
| Graph state | `ready` |
| Engine | `mere-arrangements/radial+unreachable-lane` |
| Repository cards | 16 |
| Relation text projections | 50 |
| Graph nodes | 16 |
| Graph edges | 25 |
| Horizontal overflow | 0 pixels |

Selecting Hocket changed the pressed node to `HO Hocket` while the graph
remained ready. The semantic repository cards, relationship text, legend,
controls, and metadata timestamp remained present.

## Privacy and failure behavior

The exact artifact validator rejected unapproved files, private repository
links, credential markers, email addresses, local profile paths, private
hostnames, and local or private network addresses. It scanned only reduced
public metadata and emitted hashes and counts rather than authenticated API
responses.

The metadata refresh validates a complete temporary snapshot before replacing
the last valid cache. Graph output must build reproducibly, the artifact must
match its exact approved file set, and headed smoke must pass before Pages
receives an upload. Any failure leaves the previous successful Pages deployment
in place.

## Verification

Local checks passed:

```powershell
cargo fmt --all -- --check
cargo fmt --manifest-path crates/repo-graph/Cargo.toml -- --check
cargo test --locked
cargo test --manifest-path crates/repo-graph/Cargo.toml --locked
cargo clippy --locked --all-targets -- -D warnings
cargo clippy --manifest-path crates/repo-graph/Cargo.toml --locked --all-targets -- -D warnings
cargo run --locked --bin authority -- validate-artifact . .tmp/pages-root-cutover
npm run smoke
node --check scripts/smoke-site.mjs
```

The root suite passed 16 tests across the library and M3-M5 integration
boundaries. The nested graph crate passed four tests. Workflow YAML, trigger,
and permission assertions passed. An added-line privacy scan found zero
sensitive-data matches.

## Stop

M6 is accepted. The live repository graph and organization migration plan is
complete.
