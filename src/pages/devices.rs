use std::path::Path;

use serde_json::json;

use crate::devices::{DeviceCatalog, DeviceRecord, DeviceStatus};
use crate::repositories::{AuthorityError, PublicSiteData};
use crate::site::{
    ActivePage, DEFAULT_SOCIAL_IMAGE_ALT, DEFAULT_SOCIAL_IMAGE_URL, DEVICE_CSS, DocumentMetadata,
    ORGANIZATION_ID, PageMetadata, SiteView, SocialImage, WEBSITE_ID, base_schema_graph, element,
    external_link, json_ld_for_script, link, render_with_dynamic_stylesheet,
    render_with_stylesheet, section_heading, shell, txt,
};

pub const INDEX_METADATA: PageMetadata = PageMetadata {
    title: "Radio hardware catalog | Merely",
    description: "Open radio hardware recipes ordered by product role, with demonstrated network support, installation paths, authorization records, missing work, and sale state kept separate.",
    canonical_url: "https://mer3ly.net/devices/",
};

pub fn documents(data: &PublicSiteData) -> Vec<(String, String)> {
    data.devices
        .ordered()
        .into_iter()
        .map(|device| (device.id.clone(), document_for(device)))
        .collect()
}

pub fn index_document(root: &Path) -> Result<String, AuthorityError> {
    let data = PublicSiteData::load(root)?;
    Ok(index_document_for(&data.devices))
}

pub fn index_document_for(catalog: &DeviceCatalog) -> String {
    render_with_stylesheet(
        &INDEX_METADATA,
        || index_view(catalog),
        "/devices.css",
        DEVICE_CSS,
    )
}

pub fn document(root: &Path, device_id: &str) -> Result<String, AuthorityError> {
    let data = PublicSiteData::load(root)?;
    let device = data.devices.by_id(device_id).ok_or_else(|| {
        AuthorityError::from_message(format!("unknown public device {device_id}"))
    })?;
    Ok(document_for(device))
}

pub fn document_for(device: &DeviceRecord) -> String {
    let title = format!("{} | Merely", device.name);
    let canonical = format!("https://mer3ly.net/devices/{}/", device.id);
    let json_ld = device_json_ld(device, &canonical);
    let metadata = DocumentMetadata {
        title: &title,
        description: &device.summary,
        canonical_url: &canonical,
        social_image: SocialImage {
            url: DEFAULT_SOCIAL_IMAGE_URL,
            mime_type: "image/jpeg",
            alt: DEFAULT_SOCIAL_IMAGE_ALT,
        },
        json_ld: &json_ld,
    };
    render_with_dynamic_stylesheet(
        &metadata,
        || device_view(device),
        "/devices.css",
        DEVICE_CSS,
    )
}

fn device_json_ld(device: &DeviceRecord, canonical: &str) -> String {
    let article_id = format!("{canonical}#recipe");
    let evidence_url = evidence_url(device);
    let mut graph = base_schema_graph();
    graph.push(json!({
        "@type": "WebPage",
        "@id": canonical,
        "url": canonical,
        "name": device.name,
        "description": device.summary,
        "isPartOf": { "@id": WEBSITE_ID },
        "about": { "@id": article_id }
    }));
    graph.push(json!({
        "@type": "TechArticle",
        "@id": article_id,
        "name": device.name,
        "headline": device.role,
        "description": device.summary,
        "url": canonical,
        "publisher": { "@id": ORGANIZATION_ID },
        "isBasedOn": evidence_url,
        "articleSection": ["Exact recipe state", "Build it", "Verify it", "Network support", "Install firmware", "Radio authorization", "Purchase"]
    }));
    json_ld_for_script(&json!({
        "@context": "https://schema.org",
        "@graph": graph
    }))
}

fn index_view(catalog: &DeviceCatalog) -> SiteView {
    shell(
        ActivePage::Devices,
        element(
            "main",
            &[("id", "main"), ("class", "device-catalog-main")],
            vec![
                element(
                    "section",
                    &[
                        ("class", "hero device-catalog-hero"),
                        ("aria-labelledby", "device-catalog-title"),
                    ],
                    vec![
                        element(
                            "p",
                            &[("class", "eyebrow")],
                            vec![txt("open hardware catalog")],
                        ),
                        element(
                            "h1",
                            &[("id", "device-catalog-title")],
                            vec![txt("Start with the job. Keep the recipe open.")],
                        ),
                        element(
                            "p",
                            &[("class", "hero-copy")],
                            vec![txt(concat!(
                                "Each device begins as a role and becomes an exact build recipe. ",
                                "Demonstrated networks, installable images, radio authorization, ",
                                "and sale readiness stay separate. Any purchase link comes after ",
                                "the complete DIY path.",
                            ))],
                        ),
                        catalog_flow(),
                    ],
                ),
                element(
                    "section",
                    &[("class", "content-section")],
                    vec![
                        section_heading("01", "development specimens"),
                        element(
                            "p",
                            &[("class", "section-intro")],
                            vec![txt(concat!(
                                "These are working board-level specimens. Their Reticulum, ",
                                "Meshtastic, and MeshCore receipts are stated as demonstrated ",
                                "support; their public flashing flows, authorization dossiers, ",
                                "enclosures, power assemblies, and sale readiness remain separate work.",
                            ))],
                        ),
                        element(
                            "div",
                            &[("class", "device-card-grid")],
                            catalog.ordered().into_iter().map(device_card).collect(),
                        ),
                    ],
                ),
                catalog_principle(),
            ],
        ),
    )
}

fn catalog_flow() -> SiteView {
    element(
        "ol",
        &[("class", "catalog-flow"), ("aria-label", "Catalog path")],
        [
            "choose a role",
            "build it",
            "verify it",
            "choose a network",
            "install firmware",
            "check authorization",
            "buy assembled",
        ]
        .into_iter()
        .enumerate()
        .map(|(index, label)| {
            element(
                "li",
                &[],
                vec![
                    element(
                        "span",
                        &[("aria-hidden", "true")],
                        vec![txt(format!("{:02}", index + 1))],
                    ),
                    txt(label),
                ],
            )
        })
        .collect(),
    )
}

fn device_card(device: &DeviceRecord) -> SiteView {
    let href = format!("/devices/{}/", device.id);
    let silhouette_class = if device.id.starts_with("v4-") {
        "device-silhouette device-silhouette-v4"
    } else {
        "device-silhouette device-silhouette-t114"
    };
    element(
        "article",
        &[
            ("class", "device-card"),
            ("data-device-id", device.id.as_str()),
        ],
        vec![
            element(
                "div",
                &[("class", "device-card-figure"), ("aria-hidden", "true")],
                vec![element(
                    "div",
                    &[("class", silhouette_class)],
                    vec![
                        element("span", &[("class", "device-screen")], vec![]),
                        element("span", &[("class", "device-control")], vec![]),
                        element("span", &[("class", "device-antenna")], vec![]),
                    ],
                )],
            ),
            element(
                "div",
                &[("class", "device-card-copy")],
                vec![
                    element(
                        "p",
                        &[("class", "card-kicker")],
                        vec![txt(device.status.label())],
                    ),
                    element("h2", &[], vec![txt(&device.name)]),
                    element("p", &[("class", "device-role")], vec![txt(&device.role)]),
                    element("p", &[], vec![txt(&device.summary)]),
                    element(
                        "ul",
                        &[("class", "device-card-facts")],
                        vec![
                            element("li", &[], vec![txt(&device.processor)]),
                            element("li", &[], vec![txt(&device.radio)]),
                            element("li", &[], vec![txt(&device.form)]),
                        ],
                    ),
                    link(&href, "Open the recipe and evidence", "text-link"),
                ],
            ),
        ],
    )
}

fn catalog_principle() -> SiteView {
    element(
        "section",
        &[
            ("class", "closing-note"),
            ("aria-label", "Catalog principle"),
        ],
        vec![
            element("p", &[("class", "eyebrow")], vec![txt("catalog principle")]),
            element(
                "p",
                &[("class", "closing-copy")],
                vec![txt(
                    "The assembled object is a convenience. The recipe, evidence, and right to replace its software are the product's foundation.",
                )],
            ),
        ],
    )
}

fn device_view(device: &DeviceRecord) -> SiteView {
    shell(
        ActivePage::Devices,
        element(
            "main",
            &[
                ("id", "main"),
                ("class", "device-profile-main"),
                ("data-device-id", device.id.as_str()),
                ("data-device-status", device.status.label()),
            ],
            vec![
                device_hero(device),
                recipe_section(device),
                build_section(device),
                verify_section(device),
                network_section(device),
                flash_section(device),
                authorization_section(device),
                purchase_section(device),
            ],
        ),
    )
}

fn device_hero(device: &DeviceRecord) -> SiteView {
    element(
        "section",
        &[
            ("class", "hero device-profile-hero"),
            ("aria-labelledby", "device-title"),
        ],
        vec![
            link("/devices/", "All devices", "back-link"),
            element(
                "p",
                &[("class", "eyebrow")],
                vec![txt(device.status.label())],
            ),
            element("h1", &[("id", "device-title")], vec![txt(&device.name)]),
            element(
                "p",
                &[("class", "device-profile-role")],
                vec![txt(&device.role)],
            ),
            element("p", &[("class", "hero-copy")], vec![txt(&device.summary)]),
            element(
                "div",
                &[("class", "device-status-notice")],
                vec![
                    element("strong", &[], vec![txt("Current boundary")]),
                    txt(" This is a development specimen, not a finished kit or offered product."),
                ],
            ),
        ],
    )
}

fn recipe_section(device: &DeviceRecord) -> SiteView {
    element(
        "section",
        &[("class", "content-section device-profile-section")],
        vec![
            section_heading("01", "exact recipe state"),
            element(
                "dl",
                &[("class", "device-spec-grid")],
                vec![
                    spec("Board", &device.board),
                    spec("Processor", &device.processor),
                    spec("Radio", &device.radio),
                    spec("Controls", &device.interaction),
                    spec("Power", &device.power),
                    spec("Antenna", &device.antenna),
                    spec("Enclosure", &device.enclosure),
                    spec("Form", &device.form),
                ],
            ),
        ],
    )
}

fn spec(term: &str, detail: &str) -> SiteView {
    element(
        "div",
        &[("class", "device-spec")],
        vec![
            element("dt", &[], vec![txt(term)]),
            element("dd", &[], vec![txt(detail)]),
        ],
    )
}

fn build_section(device: &DeviceRecord) -> SiteView {
    let source_url = evidence_url(device);
    element(
        "section",
        &[("class", "content-section device-profile-section")],
        vec![
            section_heading("02", "build it"),
            element(
                "p",
                &[("class", "section-lead")],
                vec![txt(&device.recipe_state)],
            ),
            element("h3", &[], vec![txt("What the complete recipe still needs")]),
            element(
                "div",
                &[("class", "requirement-list")],
                device
                    .open_requirement
                    .iter()
                    .map(|requirement| {
                        element(
                            "article",
                            &[("class", "requirement-item")],
                            vec![
                                element("h4", &[], vec![txt(&requirement.label)]),
                                element("p", &[], vec![txt(&requirement.note)]),
                            ],
                        )
                    })
                    .collect(),
            ),
            external_link(
                &source_url,
                "Read the checked hardware receipt on GitHub ↗",
                "button button-quiet",
            ),
        ],
    )
}

fn verify_section(device: &DeviceRecord) -> SiteView {
    element(
        "section",
        &[("class", "content-section device-profile-section")],
        vec![
            section_heading("03", "verify it"),
            element(
                "p",
                &[("class", "section-intro")],
                vec![txt(
                    "A proof says exactly what happened. It does not silently stand in for range, routing, loss, battery runtime, or product qualification.",
                )],
            ),
            element(
                "div",
                &[("class", "evidence-ledger")],
                device
                    .evidence
                    .iter()
                    .map(|evidence| {
                        element(
                            "article",
                            &[("class", "evidence-item")],
                            vec![
                                element(
                                    "p",
                                    &[("class", "evidence-state")],
                                    vec![txt(evidence.state.label())],
                                ),
                                element("h3", &[], vec![txt(&evidence.label)]),
                                element("p", &[], vec![txt(&evidence.note)]),
                            ],
                        )
                    })
                    .collect(),
            ),
        ],
    )
}

fn network_section(device: &DeviceRecord) -> SiteView {
    element(
        "section",
        &[("class", "content-section device-profile-section")],
        vec![
            section_heading("04", "network support"),
            element(
                "p",
                &[("class", "section-intro")],
                vec![txt(concat!(
                    "This ledger records demonstrated network behavior. It is independent of ",
                    "whether a one-click installation recipe or assembled product is ready.",
                ))],
            ),
            element(
                "div",
                &[("class", "catalog-choice-grid")],
                device
                    .network_support
                    .iter()
                    .map(|network| {
                        element(
                            "article",
                            &[("class", "catalog-choice")],
                            vec![
                                element(
                                    "p",
                                    &[("class", "evidence-state")],
                                    vec![txt(network.state.label())],
                                ),
                                element("h3", &[], vec![txt(&network.name)]),
                                element("p", &[], vec![txt(&network.note)]),
                            ],
                        )
                    })
                    .collect(),
            ),
        ],
    )
}

fn flash_section(device: &DeviceRecord) -> SiteView {
    element(
        "section",
        &[("class", "content-section device-profile-section")],
        vec![
            section_heading("05", "install firmware"),
            element(
                "p",
                &[("class", "section-intro")],
                vec![txt(concat!(
                    "Firmware belongs to the owner. Installation recipes are tracked separately ",
                    "from network support, and one SX1262 radio runs one selected personality at a time.",
                ))],
            ),
            element(
                "div",
                &[("class", "catalog-choice-grid")],
                device
                    .flash_recipe
                    .iter()
                    .map(|recipe| {
                        element(
                            "article",
                            &[("class", "catalog-choice")],
                            vec![
                                element(
                                    "p",
                                    &[("class", "evidence-state")],
                                    vec![txt(recipe.state.label())],
                                ),
                                element("h3", &[], vec![txt(&recipe.name)]),
                                element("p", &[], vec![txt(&recipe.note)]),
                            ],
                        )
                    })
                    .collect(),
            ),
        ],
    )
}

fn authorization_section(device: &DeviceRecord) -> SiteView {
    element(
        "section",
        &[("class", "content-section device-profile-section")],
        vec![
            section_heading("06", "radio authorization"),
            element(
                "div",
                &[("class", "device-status-notice")],
                vec![
                    element("strong", &[], vec![txt(device.authorization.state.label())]),
                    txt(format!(" {}", device.authorization.note)),
                ],
            ),
            element(
                "dl",
                &[("class", "device-spec-grid authorization-grid")],
                vec![
                    spec("Exact device", &device.authorization.device),
                    spec("Antenna conditions", &device.authorization.antenna),
                    spec(
                        "Operating envelope",
                        &device.authorization.operating_envelope,
                    ),
                ],
            ),
        ],
    )
}

fn purchase_section(device: &DeviceRecord) -> SiteView {
    let mut contents = vec![
        section_heading("07", "buy assembled hardware"),
        element(
            "p",
            &[("class", "section-lead")],
            vec![txt(
                "The purchase control comes last, after the recipe, evidence, network support, installation paths, and radio authorization.",
            )],
        ),
    ];
    match (&device.status, &device.sale.purchase_url) {
        (DeviceStatus::Sellable, Some(url)) => contents.push(external_link(
            url,
            "Buy this assembled device ↗",
            "button button-primary purchase-link",
        )),
        _ => contents.push(element(
            "div",
            &[
                ("class", "purchase-unavailable"),
                ("data-purchase-status", "unavailable"),
            ],
            vec![
                element("strong", &[], vec![txt(device.sale.state.label())]),
                element("p", &[], vec![txt(&device.sale.note)]),
            ],
        )),
    }
    element(
        "section",
        &[(
            "class",
            "content-section device-profile-section device-purchase-section",
        )],
        contents,
    )
}

fn evidence_url(device: &DeviceRecord) -> String {
    format!(
        "{}/blob/main/{}",
        device.source_repository, device.source_document
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn tech_article_does_not_emit_an_offer() {
        let data = PublicSiteData::load(env!("CARGO_MANIFEST_DIR")).expect("public site data");
        let device = data.devices.ordered()[0];
        let canonical = format!("https://mer3ly.net/devices/{}/", device.id);
        let encoded = device_json_ld(device, &canonical);
        let value: Value = serde_json::from_str(&encoded).expect("valid JSON-LD");
        assert_eq!(value["@context"], "https://schema.org");
        assert!(!encoded.contains("\"offers\""));
    }
}
