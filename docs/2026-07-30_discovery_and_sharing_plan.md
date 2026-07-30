# Merely discovery and sharing

**Status:** M8 complete; accepted by the
[discovery and sharing receipt](receipts/site/2026-07-30_m8_discovery_and_sharing.md)

**Authority:** This is the canonical plan for making the completed Mer3ly
project profiles independently discoverable and shareable. It follows the
accepted [M7 project showcase](receipts/site/2026-07-30_m7_project_showcase.md)
without reopening repository ownership, project copy, or deployment.

## Purpose

M7 gave every public repository a truthful local profile. M8 makes those
profiles legible to crawlers, link unfurlers, and browser identity surfaces
without introducing a content framework or a second source of truth.

The generated static document remains authoritative. Discovery files and head
metadata are projections of the same committed repository, relation, metadata,
and showcase records used by the visible pages.

## Standards boundary

- `sitemap.xml` follows the
  [Sitemaps XML protocol](https://www.sitemaps.org/protocol.html) and contains
  only canonical `https://mer3ly.net` URLs.
- `robots.txt` allows the public site and names the canonical sitemap.
- Social metadata follows the
  [Open Graph protocol](https://ogp.me/), including an image description.
- Project structured data uses
  [Schema.org SoftwareSourceCode](https://schema.org/SoftwareSourceCode) when
  the profile represents a software repository. The organization profile uses
  `CreativeWork` rather than pretending to be software.
- The site gains an SVG favicon. It does not gain a web-app manifest, service
  worker, or install prompt because Mer3ly is a website rather than an
  installable application.

## Metadata model

Every generated HTML document retains:

- one title and description;
- one canonical URL;
- Open Graph title, type, URL, description, image, image MIME type, and image
  alt text;
- matching Twitter card title, description, image, and image alt text;
- the Merely site name and theme color;
- a favicon link.

Fixed pages use the existing Merely social image. A project with an approved
showcase uses its normalized screenshot and exact approved alt text. A
text-first project uses the Merely fallback image and a plain site-level image
description.

Project JSON-LD contains a bounded `@graph`:

- Merely LLC as the publisher;
- Mer3ly as the containing website;
- the canonical project profile as a web page;
- the repository as `SoftwareSourceCode`, or `CreativeWork` for the
  organization profile;
- public repository URL, summary, and primary language when recorded.

JSON-LD is serialized from validated authority. Values are escaped for an HTML
script context before insertion.

## Discovery files

Generate a root `sitemap.xml` containing:

- `/`;
- `/repos/`;
- `/radio.html`;
- one `/projects/<repository-id>/` URL for every public repository.

M8 omits `lastmod`, `changefreq`, and `priority`. The current authority does not
contain a truthful page-modification timestamp, and crawler hints should not be
manufactured from build time or GitHub activity.

Generate a root `robots.txt` containing:

```text
User-agent: *
Allow: /
Sitemap: https://mer3ly.net/sitemap.xml
```

## Implementation seams

- `src/site.rs` owns safe head rendering and the fixed Merely identity.
- `src/pages/projects.rs` derives project social images and structured data
  from validated public site data.
- `src/discovery.rs` generates the sitemap and robots policy.
- `src/main.rs` writes the discovery files and favicon into the site root.
- `src/artifact.rs` enforces the exact file set, canonical URL coverage,
  project metadata, structured-data shape, favicon identity, and discovery
  file contents.
- `scripts/smoke-site.mjs` serves and inspects the new MIME types and verifies
  representative visual and text-first metadata in Chromium.

## Milestones

### M8A: safe metadata model

- Add social-image fields and image descriptions to head rendering.
- Add matching Twitter metadata.
- Add project-specific JSON-LD and social-image selection.

**Done when:** every project has exactly one canonical identity, one described
social image, parseable JSON-LD, and a truthful visual or fallback boundary.

### M8B: crawler and browser identity

- Generate `sitemap.xml` and `robots.txt`.
- Add and link the SVG favicon.
- Keep the sitemap limited to canonical public documents.

**Done when:** the sitemap has exactly 19 unique same-origin URLs, the robots
policy names it, and every HTML page links the checked favicon.

### M8C: acceptance

- Extend exact-artifact validation and receipt counts.
- Add Rust integration tests and headed browser checks.
- Rebuild committed `html/`.
- Scan staged and generated output for sensitive information.
- Commit in logical batches, push, deploy, and compare the public edge with the
  accepted artifact under browser-navigation requests.

**Done when:** source checks, artifact validation, headed smoke, Pages
deployment, beacon-free HTML, HSTS, and byte-for-byte public comparison pass.

## Stop rules

- Do not add analytics, tracking pixels, external preview-image services, or
  crawler submission credentials.
- Do not add a CMS, client-side router, service worker, web-app manifest, or
  second deployment origin.
- Do not publish Mesocosm or Paredros before public remotes and metadata exist.
- Do not implement cross-repository refresh dispatch in M8.
- Do not claim sitemap freshness that the authority cannot prove.
