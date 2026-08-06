# Device catalog slice

## Purpose

Add a public hardware catalog that begins with what a person wants the device to do, discloses the exact known build state, and puts any purchase link after the DIY material. The first records are evidence-backed development specimens, not finished products.

## Authority boundary

- Mer3ly owns the public catalog record, route, wording, and sale status.
- Retinue remains the firmware and hardware-evidence source. Catalog records link to its public receipts rather than copying them into a second authority.
- The future flashing app may consume a shared device or personality manifest. That extraction waits until it is a real second consumer.

## Public model

Each device record names:

- its role and physical form;
- the board, processor, radio, controls, power state, and enclosure state;
- demonstrated network support, independent of installation packaging;
- owner-selectable firmware images and the current flash-recipe boundary;
- an exact-device radio authorization record, separate from software licensing;
- proven, partial, and open evidence;
- missing work that prevents a complete recipe or sale;
- an optional purchase URL, allowed only for a sellable record.

Status progresses through `candidate`, `development-specimen`, `proven-recipe`, `assembled-prototype`, and `sellable`. Status is evidence, not marketing copy.

## First routes

- `/devices/` lists the catalog by product role and status.
- `/devices/v4-desktop-radio/` documents the Heltec V4 development specimen.
- `/devices/t114-field-radio/` documents the LilyGo T114 development specimen.

Every detail page is ordered: role, exact recipe state, build, verify, network support, install firmware, radio authorization, then purchase. For this slice, the purchase section explicitly says the device is not offered for sale.

## Acceptance receipts

- `content/devices.toml` passes strict schema, slug, source-link, network-support, flash-recipe, authorization, status, and purchase-state validation.
- Both records disclose missing enclosure, power, antenna, and sale-readiness work instead of inventing specifications.
- Static generation emits the index and both profiles, includes them in the sitemap, and exposes them in the main navigation.
- Artifact validation requires exactly those public routes and rejects purchase links on non-sellable profiles.
- Unit and integration tests cover manifest failures, catalog rendering, metadata, and discovery.

## Stop rules

- Do not add prices, batteries, antennas, range, power-output claims, or enclosure files without a checked recipe and evidence.
- Do not describe one-hop direct-PHY carriage as mesh, routing, loss, or range proof.
- Do not imply simultaneous firmware personalities on a single SX1262 radio.
- Do not use an unfinished public flash recipe to downgrade demonstrated protocol interoperability.
- Do not treat software licensing, protocol support, or a regional firmware cap as equipment authorization for a catalog SKU.
- Do not add checkout, inventory, fulfillment, or a purchase URL before a device reaches `sellable` and receives a separate compliance review.
- Do not publish local paths, machine identifiers, private-network addresses, or private receipts.
