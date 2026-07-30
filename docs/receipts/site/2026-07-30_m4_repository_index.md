# M4 semantic repository index

**Date:** 2026-07-30 UTC

**Implementation commit:** `0b1ddff`

**Pages run:** `30518337035`

**Result:** authority, cache, static-output, privacy, responsive-browser, topic,
and public-deployment checks accepted. M5 was not started.

## What changed

- `src/pages/repositories.rs` renders `/repos/` from the typed repository
  authority and the reduced public GitHub cache.
- `src/repositories.rs` validates one public metadata record for every public
  authority repository and rejects missing, duplicate, extra, or malformed
  records.
- `scripts/refresh-public-metadata.ps1` writes a complete temporary snapshot,
  validates it through Rust, and replaces the committed cache only after every
  public query succeeds.
- `content/github-metadata.json` contains only repository identity, public
  timestamps, primary language, stars, fork/archive state, and topics.
- Native radio controls provide class filters. The complete repository list
  and all relationship text remain present without JavaScript or Wasm.

## Authority projection

The generated page contains:

| Record | Count |
| --- | ---: |
| Public repositories | 16 |
| Typed relationships | 25 |
| Derived relationships | 22 |
| Curated relationships | 3 |
| Relation text projections | 50 |

Every relation appears once under its source repository and once under its
target repository. Each occurrence keeps the same stable relation id and its
derived or curated provenance.

The cache uses schema `mer3ly.github-metadata/v1`, contains 16 records, and
records refresh time `2026-07-30T05:44:43Z`. Its size is 7,290 bytes.

A forced failed refresh left the existing cache unchanged:

```text
before  4CF9730ED84FACE29719CF8E1357E7EC34E89B1475D5B65FD5FF094E0C5CD655
after   4CF9730ED84FACE29719CF8E1357E7EC34E89B1475D5B65FD5FF094E0C5CD655
```

## Repository topics

All 16 public `merely-made` repositories have a bounded topic set:

| Repository | Topics |
| --- | --- |
| `.github` | `merely-made`, `open-source`, `organization-profile` |
| `genet` | `browser-engine`, `rust`, `servo`, `vello`, `web-engine`, `web-platform-tests`, `wgpu` |
| `hocket` | `audio`, `audio-looper`, `cooperative`, `local-first`, `peer-to-peer`, `rust` |
| `isometry` | `isometric`, `local-first`, `peer-to-peer`, `pixel-art`, `rust`, `virtual-tabletop` |
| `mer3ly-net` | `cambium`, `genet`, `local-first`, `open-source`, `rust`, `static-site` |
| `mere` | `browser`, `graph`, `graph-gui`, `knowledge-graph`, `local-first`, `peer-to-peer`, `rust`, `wgpu` |
| `netrender` | `renderer`, `rust`, `servo`, `vello`, `web-rendering`, `wgpu` |
| `retinue` | `embedded`, `lora`, `mesh-networking`, `offline-first`, `peer-to-peer`, `reticulum`, `rust` |
| `smolweb` | `guppy-protocol`, `misfin`, `nex-protocol`, `protocols`, `rust`, `small-web`, `spartan-protocol` |
| `turnstone` | `browser`, `cooperative-browsing`, `graph-browser`, `local-first`, `peer-to-peer`, `rust`, `wgpu` |
| `vano` | `data-oriented-design`, `ecmascript`, `genet`, `javascript-engine`, `nova`, `rust` |
| `wavicle` | `audio-codec`, `codec`, `lossless-audio`, `rust`, `wavpack` |
| `wgpu-graft` | `embedding`, `gpu`, `graphics`, `rust`, `texture`, `wgpu` |
| `wgpu-scry` | `embedding`, `gpu`, `rust`, `texture`, `webview`, `wgpu` |
| `wgpu-weld` | `cef`, `chromium-embedded-framework`, `embedding`, `gpu`, `rust`, `texture`, `wgpu` |
| `woodshed` | `local-first`, `metronome`, `music-practice`, `music-theory`, `musicians`, `rust`, `tuner` |

The same pass corrected Mer3ly's homepage and description, Mere's homepage,
and Genet's description. Genet now describes its profiles without making an
unsupported standards-compliance claim.

## Static output

| Artifact | Bytes |
| --- | ---: |
| `html/index.html` | 5,657 |
| `html/radio.html` | 7,909 |
| `html/repos/index.html` | 42,967 |
| `html/site.css` | 22,579 |
| `content/github-metadata.json` | 7,290 |
| `content/repositories.toml` | 4,830 |
| `content/relations.toml` | 6,242 |
| Conservative HTML, CSS, and repository-data total | 97,474 |

The 97,474-byte total is below the 200 KiB limit. It conservatively counts the
authority and cache separately even though their public information is already
projected into the generated page.

## Verification

These commands passed:

```powershell
cargo fmt --all -- --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo run --locked --bin authority -- validate
cargo run --locked --bin authority -- validate-metadata
cargo run --locked --bin site
```

The Rust suite ran ten tests. It covers exact repository and edge projection,
Genet parsing, static semantics, class filters, native keyboard focus
contracts, cache reduction, privacy markers, and the size boundary.

The local Genet layout receipt produced these absolute right edges:

| Page | 375 px | 420 px | 900 px | 1440 px |
| --- | ---: | ---: | ---: | ---: |
| Repositories | 375.00 | 420.00 | 900.00 | 1440.00 |

The page produced 864 fragments at every width. The receipt used the live
local Genet layout checkout and emitted nine existing `genet-layout` warnings;
no warning came from Mer3ly.

## Headed browser review

At each width, the document's `scrollWidth` equaled its `clientWidth`:

| Viewport | Client width | Scroll width | Cards |
| --- | ---: | ---: | ---: |
| 375 | 360 | 360 | 16 |
| 420 | 405 | 405 | 16 |
| 900 | 885 | 885 | 16 |
| 1440 | 1,425 | 1,425 | 16 |

Fonts loaded at every width. Headed interaction returned the expected visible
card counts: all 16, products 5, platforms 3, foundations 7, and tools 1. The
accessibility snapshot exposes the filter controls as named native radios and
the card collection as a named repository-index region.

Required captures:

- [375-pixel page](m4/repos-375.png)
- [420-pixel page](m4/repos-420.png)
- [900-pixel page](m4/repos-900.png)
- [1440-pixel page](m4/repos-1440.png)
- [Genet relationships at 375](m4/repos-375-relations.png)
- [Genet relationships at 900](m4/repos-900-relations.png)
- [Live page at 900](m4/live-repos-900.png)

## Privacy

The staged additions were scanned for machine-profile paths, account markers,
loopback and private-network addresses, email addresses, phone numbers,
credential shapes, private-key blocks, and assigned secrets. The only raw
path match was the refresh script's own `/Users/` rejection pattern. After
excluding that detector literal, the scan reported zero findings.

The refresh script independently rejects absolute paths, `file://` URLs,
authenticated-only fields, SSH URLs, and token-shaped field names before
validation. No GitHub credential reaches the cache, generated HTML, or
browser.

## Deployment

GitHub's Pages build for `0b1ddff` completed successfully in run
`30518337035`. The Cloudflare-fronted public page returned:

| URL | Result |
| --- | --- |
| `https://mer3ly.net/repos/` | 200, 16 cards, 50 relation projections |
| `http://mer3ly.net/repos/` | 200, no redirect |

The live headed page loaded all fonts and reported equal 885-pixel client and
scroll widths in a 900-pixel viewport. HTTP-to-HTTPS enforcement remains the
same external Cloudflare configuration gate recorded at M3.

## Stop

M4 source, repository topics, and public deployment are accepted. The live
Mere graph in M5 was not started.
