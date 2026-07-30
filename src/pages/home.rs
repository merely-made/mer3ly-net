use crate::site::{
    ActivePage, PageMetadata, SiteView, element, external_link, link, render_with, section_heading,
    shell, txt,
};

pub const METADATA: PageMetadata = PageMetadata {
    title: "Merely | Local-first software and community radio",
    description: "Merely builds local-first software, graph-shaped web tools, and community-owned radio infrastructure in Ashland, Kentucky.",
    canonical_url: "https://mer3ly.net/",
};

pub fn document() -> String {
    render_with(&METADATA, view)
}

pub fn view() -> SiteView {
    shell(
        ActivePage::Home,
        element(
            "main",
            &[("id", "main")],
            vec![hero(), radio_feature(), projects(), principle()],
        ),
    )
}

fn hero() -> SiteView {
    element(
        "section",
        &[
            ("class", "hero home-hero"),
            ("aria-labelledby", "home-title"),
        ],
        vec![
            element(
                "p",
                &[("class", "eyebrow")],
                vec![txt("Merely LLC · Ashland, Kentucky")],
            ),
            element(
                "h1",
                &[("id", "home-title")],
                vec![txt("Tools for people who are their own infrastructure.")],
            ),
            element(
                "p",
                &[("class", "hero-copy")],
                vec![txt(
                    "We build local-first software, graph-shaped web tools, and community-owned radio systems. The work is open, inspectable, and made to keep working on your own terms.",
                )],
            ),
            element(
                "div",
                &[("class", "hero-actions")],
                vec![
                    link(
                        "/radio.html",
                        "Read about community radio",
                        "button button-primary",
                    ),
                    link(
                        "/repos/",
                        "Explore the repository map",
                        "button button-quiet",
                    ),
                ],
            ),
            element(
                "div",
                &[("class", "signal-rule"), ("aria-hidden", "true")],
                vec![],
            ),
        ],
    )
}

fn radio_feature() -> SiteView {
    element(
        "section",
        &[("class", "content-section")],
        vec![
            section_heading("01", "community radio"),
            element(
                "article",
                &[("class", "feature-card")],
                vec![
                    element(
                        "div",
                        &[("class", "feature-copy")],
                        vec![
                            element(
                                "p",
                                &[("class", "card-kicker")],
                                vec![txt("Retinue · field status")],
                            ),
                            element(
                                "h3",
                                &[],
                                vec![txt("Radios that work when the towers do not.")],
                            ),
                            element(
                                "p",
                                &[],
                                vec![txt(
                                    "Small LoRa radios relay encrypted messages device to device without cell towers, internet service, or a monthly bill. Three units are exchanging data on the workbench today. The next field target is a ten-site FIVCO pilot.",
                                )],
                            ),
                            link("/radio.html", "Read the pilot brief", "text-link"),
                        ],
                    ),
                    element(
                        "dl",
                        &[("class", "status-list")],
                        vec![
                            status("radios on air", "3"),
                            status("mesh standards", "3"),
                            status("unit cost", "~ a tank of gas"),
                            status("monthly bill", "none"),
                        ],
                    ),
                ],
            ),
        ],
    )
}

fn status(term: &str, value: &str) -> SiteView {
    element(
        "div",
        &[("class", "status-row")],
        vec![
            element("dt", &[], vec![txt(term)]),
            element("dd", &[], vec![txt(value)]),
        ],
    )
}

fn projects() -> SiteView {
    element(
        "section",
        &[("class", "content-section")],
        vec![
            section_heading("02", "what we build"),
            element(
                "div",
                &[("class", "project-grid")],
                vec![
                    project(
                        "Retinue",
                        "Digital-radio transport: mesh wireless standards today, with a broad hardware base as the destination.",
                        "https://github.com/merely-made/retinue",
                    ),
                    project(
                        "Turnstone",
                        "A graph-shaped browser: pages, media, and notes live as nodes you can relate, arrange, and revisit.",
                        "https://github.com/merely-made/turnstone",
                    ),
                    project(
                        "Mere",
                        "The modular graph-browser library for graph truth, arrangement, memory, retrieval, identity, and peer layers.",
                        "https://github.com/merely-made/mere",
                    ),
                    project(
                        "Genet",
                        "A Servo-derived, data-oriented web engine family for rendering web content inside Merely applications.",
                        "https://github.com/merely-made/genet",
                    ),
                ],
            ),
        ],
    )
}

fn project(name: &str, description: &str, href: &str) -> SiteView {
    element(
        "article",
        &[("class", "project-card")],
        vec![
            element("h3", &[], vec![txt(name)]),
            element("p", &[], vec![txt(description)]),
            external_link(href, "repository ↗", "text-link"),
        ],
    )
}

fn principle() -> SiteView {
    element(
        "section",
        &[
            ("class", "closing-note"),
            ("aria-label", "Working principle"),
        ],
        vec![
            element("p", &[("class", "eyebrow")], vec![txt("working principle")]),
            element(
                "p",
                &[("class", "closing-copy")],
                vec![txt(
                    "Your tools should explain themselves, preserve your data, and remain useful when somebody else's service is unavailable.",
                )],
            ),
        ],
    )
}
