# M3 static Cambium/Genet site

**Date:** 2026-07-30 UTC
**Implementation commit:** `b3e99e4`
**Result:** code, static-output, privacy, layout, headed-browser, and public
deployment checks accepted. The Cloudflare HTTPS redirect gate was closed
later the same day. M4 was not started within this milestone.

## What changed

- `src/pages/` owns the Cambium home and community-radio views.
- `src/site.rs` owns the shared shell, navigation, footer, metadata, structured
  data, and Genet serialization.
- `src/main.rs` writes the deployment artifact to `html/`.
- `assets/site.css` owns the responsive visual system.
- `assets/og.jpg` is the 1200 by 630 social preview.
- `html/` contains generated HTML, CSS, image, and `CNAME` files.

The pages contain no client-side application runtime. The only `script`
element is static `application/ld+json` organization data.

## Static output

| Artifact | Bytes |
| --- | ---: |
| `html/index.html` | 5,630 |
| `html/radio.html` | 7,849 |
| `html/site.css` | 14,546 |
| Baseline HTML and CSS | 28,025 |
| `html/og.jpg` | 177,124 |

The 28,025-byte baseline is below the M3 limit of 200 KiB. Images and external
fonts are excluded from that limit as specified by the plan. The previous
roughly 1.1 MB generated runtime bundle and its component manifest are absent.

## Verification

These commands passed:

```powershell
cargo fmt --all -- --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo run --locked --bin authority -- validate
cargo run --locked --bin authority -- summary
```

The Rust suite ran six tests. It covers Genet parse/serialization, semantic
landmarks, metadata, the static-size budget, privacy/runtime markers,
responsive CSS contracts, and the existing repository authority.

The local Genet layout receipt produced these absolute right edges:

| Page | 375 px | 420 px | 900 px | 1440 px |
| --- | ---: | ---: | ---: | ---: |
| Home | 375.00 | 420.00 | 900.00 | 1440.00 |
| Radio | 375.00 | 420.00 | 900.00 | 1440.00 |

Home produced 93 fragments and radio produced 139 at each width. The receipt
used the live local Genet layout checkout and emitted nine existing
`genet-layout` warnings; no warning came from Mer3ly.

## Headed browser review

At 375, 420, 900, and 1440 CSS pixels, each page reported equal document
`scrollWidth` and `clientWidth`. Fonts loaded at every width. The headed pass
caught and fixed a narrow home-heading wrap and a 375-pixel closing-card label
collision before acceptance.

Required width captures:

- Home:
  [375](m3/home-375.jpg),
  [420](m3/home-420.jpg),
  [900](m3/home-900.jpg),
  [1440](m3/home-1440.jpg)
- Radio:
  [375](m3/radio-375.jpg),
  [420](m3/radio-420.jpg),
  [900](m3/radio-900.jpg),
  [1440](m3/radio-1440.jpg)

Focused lower-page captures:

- [Radio mesh at 900](m3/radio-900-mesh.jpg)
- [Radio pilot at 900](m3/radio-900-pilot.jpg)
- [Radio costs at 375](m3/radio-375-costs.jpg)
- [Radio closing cards at 375](m3/radio-375-closing.jpg)

Public deployment captures:

- [Live home at 900](m3/live-home-900.jpg)
- [Live radio at 900](m3/live-radio-900.jpg)

All capture files are JPEG images.

## Privacy and public contact

The staged additions and live documents were scanned for machine-profile
paths, private-network addresses, credentials, personal names, phone numbers,
and email addresses. No matches remained. The old generated footer's personal
details are gone.

The footer intentionally exposes only:

- Merely LLC;
- Ashland, Kentucky;
- `mer3ly.net`; and
- the public `merely-made` GitHub organization as the contact channel.

The design-system export used for reference was moved outside the repository
to a local archive. It is not part of the commit or deployment.

## Deployment

GitHub's legacy Pages build for `b3e99e4` completed successfully in run
`30510259467`. The Cloudflare-fronted public domain then returned:

| URL | Result |
| --- | --- |
| `https://mer3ly.net/` | 200, new 5,630-byte home document |
| `https://mer3ly.net/radio` | 200, new 7,849-byte radio document |
| `https://mer3ly.net/site.css` | 200, 14,546-byte stylesheet |
| `https://mer3ly.net/og.jpg` | 200, 177,124-byte JPEG |

At initial deployment, HTTPS transport worked but was not enforced:
`http://mer3ly.net/` also returned 200. GitHub reported
`https_enforced: false`, and its Pages API rejected enforcement because the
GitHub-side certificate did not exist while the domain resolved through
Cloudflare. The remaining gate therefore belonged to Cloudflare rather than
the repository.

## HTTPS gate closure

Cloudflare's `Always Use HTTPS` setting was enabled later on 2026-07-30.
Independent edge requests then returned:

| URL | Result |
| --- | --- |
| `http://mer3ly.net/` | 301 to `https://mer3ly.net/` |
| `http://mer3ly.net/radio` | 301 to `https://mer3ly.net/radio` |
| `http://mer3ly.net/repos/` | 301 to `https://mer3ly.net/repos/` |
| All three HTTPS destinations | 200 |

## Stop

M3 source and public deployment are fully accepted. M4, the semantic
repository page, was not started within this milestone.
