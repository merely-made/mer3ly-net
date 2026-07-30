# M8 discovery and sharing

**Accepted:** 2026-07-30

**Source:** `cdd7a04385061f282b5b878f4c49961f6ae690e8`

**Deployment run:** [30587753072](https://github.com/merely-made/mer3ly-net/actions/runs/30587753072)

## Result

Crawler discovery, project-specific social previews, bounded structured data,
and browser identity are accepted on the public Mer3ly site.

The implementation retains the static site boundary. It adds no CMS, router,
service worker, web-app manifest, analytics, crawler credentials, or external
preview service.

## Checked artifact

The successful workflow built and deployed a 33-file Pages artifact totaling
1,706,172 bytes. The Pages upload was artifact `8777312991`; headed and static
acceptance evidence was artifact `8777312683`.

The artifact validator accepted:

- 16 public repositories and 25 relationship edges;
- 16 project profiles and 50 project relationship projections;
- 5 approved showcase images;
- 19 unique canonical sitemap URLs;
- 16 project social-preview records;
- 16 project structured-data records;
- refreshed public metadata generated at `2026-07-30T22:40:59Z`, with SHA-256
  `197c3aa5239a82a063965ecff315bc07992e6b515a4a10cf17ac83af4b55a697`.

The exact artifact contains `sitemap.xml`, `robots.txt`, and `favicon.svg`.
The sitemap contains only canonical `https://mer3ly.net` documents and omits
manufactured `lastmod`, `changefreq`, and `priority` values. The robots policy
allows the public site and names the canonical sitemap.

## Metadata boundary

All 19 HTML documents have one canonical URL, a described Open Graph image,
matching Twitter image metadata, a favicon link, a sitemap link, and one
parseable JSON-LD record.

Projects with approved showcase evidence use their normalized screenshot as
the social image. Text-first projects use the Merely fallback image. The
headed receipt specifically accepted Mere at
`https://mer3ly.net/showcase/mere.png` and Retinue at
`https://mer3ly.net/og.jpg`.

Software project records use `SoftwareSourceCode` with their public repository
URL. The organization profile uses `CreativeWork`. All project pages name
Merely LLC as publisher and Mer3ly as the containing website.

The artifact validator binds the displayed refresh receipt to the exact
sentence in the repository page. A replay against stale local metadata was
rejected even though an unrelated repository update happened to share the
same minute.

## Browser acceptance

True-headed Chromium `151.0.7922.34` accepted:

- all representative routes at HTTP 200 with one `h1` and no horizontal
  overflow;
- 19 sitemap URLs, the public robots policy, and the SVG favicon;
- the Mere visual profile and Retinue text-first profile metadata;
- 5 decoded showcase images at desktop and mobile widths;
- 16 graph nodes, 25 graph edges, and 50 semantic relationship projections;
- the mobile WebGPU graph path;
- desktop and reduced-motion fallback paths when WebGPU was unavailable;
- the forced no-WebGPU fallback.

The local visible-browser pass also rendered the home and Mere project pages
without console errors or horizontal overflow. Mere exposed the expected
showcase social image, image description, source repository, and
`SoftwareSourceCode` record.

## Public edge

After deployment, browser-shaped requests fetched every deployable artifact
file from `https://mer3ly.net`. All 32 public files matched the accepted Pages
artifact byte for byte. `CNAME` is deployment metadata and is not counted as a
public route.

The edge returned:

- `image/svg+xml` for `favicon.svg`;
- `text/plain` for `robots.txt`;
- `application/xml` for `sitemap.xml`;
- `Strict-Transport-Security: max-age=2592000`.

All 19 public HTML files were free of Cloudflare Web Analytics beacon
injection.

## Privacy and source checks

The exact-artifact validator scanned every generated text asset for secrets,
unapproved contact addresses, private GitHub links, local filesystem paths,
private hostnames, and private network addresses. The staged diff received a
second marker scan. The only source hits were the validator's forbidden-marker
list and its synthetic rejection fixture.

No sensitive output required a new ignore rule. `markik@mer3ly.net` remains the
single approved public contact address.

Local verification passed:

```powershell
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo test --manifest-path crates/repo-graph/Cargo.toml --locked
cargo run --locked --bin authority -- validate
cargo run --locked --bin authority -- validate-metadata
cargo run --locked --bin authority -- validate-artifact . html
node --check scripts/smoke-site.mjs
npm run smoke
```

The root suite passed 28 tests. The repository-graph crate passed 4 more. A
fresh generated artifact matched committed `html/` across all 33 files.

## Acceptance

M8 is accepted. Discovery and social metadata remain projections of committed
repository, relation, metadata, and showcase authority. Mesocosm and Paredros
remain outside the public map until public remotes and validated metadata
exist.
