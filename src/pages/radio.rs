use crate::site::{
    ActivePage, PageMetadata, SiteView, element, external_link, render_with, section_heading,
    shell, txt,
};

pub const METADATA: PageMetadata = PageMetadata {
    title: "Community radio | Merely",
    description: "A low-cost, open-source LoRa radio pilot for community-owned backup communications across the FIVCO counties.",
    canonical_url: "https://mer3ly.net/radio.html",
};

pub fn document() -> String {
    render_with(&METADATA, view)
}

pub fn view() -> SiteView {
    shell(
        ActivePage::Radio,
        element(
            "main",
            &[("id", "main"), ("class", "radio-main")],
            vec![
                hero(),
                problem_solution(),
                mesh(),
                pilot(),
                costs(),
                partnership(),
            ],
        ),
    )
}

fn hero() -> SiteView {
    element(
        "header",
        &[("class", "hero radio-hero")],
        vec![
            element(
                "p",
                &[("class", "eyebrow")],
                vec![txt("Retinue · community radio")],
            ),
            element(
                "h1",
                &[],
                vec![txt(
                    "Low-cost radio networks that keep communities messaging when cell towers and internet go down.",
                )],
            ),
            element(
                "p",
                &[("class", "hero-copy")],
                vec![txt(
                    "A project of Merely LLC in Ashland, Kentucky. Open standards, open source, community-owned.",
                )],
            ),
        ],
    )
}

fn problem_solution() -> SiteView {
    element(
        "section",
        &[("class", "two-up"), ("aria-label", "Problem and approach")],
        vec![
            numbered_card(
                "01",
                "the problem",
                "Storms, floods, and ice take down power and communications in our region, sometimes for days. Families cannot reach each other, and volunteer responders lose coordination exactly when they need it most.",
            ),
            numbered_card(
                "02",
                "the approach",
                "Small LoRa radios relay data device to device at long range. Hosted at fire stations, churches, ridgelines, and public facilities, they form local networks that can link with their neighbors.",
            ),
        ],
    )
}

fn numbered_card(number: &str, heading: &str, copy: &str) -> SiteView {
    element(
        "article",
        &[("class", "info-card")],
        vec![
            element(
                "p",
                &[("class", "card-kicker")],
                vec![txt(format!("{number} · {heading}"))],
            ),
            element("p", &[], vec![txt(copy)]),
        ],
    )
}

fn mesh() -> SiteView {
    element(
        "section",
        &[("class", "content-section")],
        vec![
            section_heading("03", "how the mesh works"),
            element(
                "figure",
                &[("class", "mesh-card")],
                vec![
                    mesh_diagram(),
                    mesh_diagram_mobile(),
                    element(
                        "figcaption",
                        &[],
                        vec![txt(
                            "When a direct path is unavailable, a message follows the relays that can still hear one another.",
                        )],
                    ),
                    element(
                        "p",
                        &[],
                        vec![txt(
                            "Each unit costs about as much as a tank of gas and can run on battery, solar, or wall power. The network remains useful as long as working radios retain a path between them.",
                        )],
                    ),
                ],
            ),
        ],
    )
}

fn mesh_diagram() -> SiteView {
    element(
        "svg",
        &[
            ("class", "mesh-diagram mesh-diagram-desktop"),
            ("viewBox", "0 0 800 240"),
            ("role", "img"),
            (
                "aria-labelledby",
                "mesh-diagram-title mesh-diagram-description",
            ),
        ],
        vec![
            element(
                "title",
                &[("id", "mesh-diagram-title")],
                vec![txt("A message relayed around a blocked direct path")],
            ),
            element(
                "desc",
                &[("id", "mesh-diagram-description")],
                vec![txt(
                    "Five radios at a fire station, church steeple, water tower, ridgeline, and county garage form alternate routes through the mesh.",
                )],
            ),
            svg_line("110", "170", "270", "80", "mesh-link"),
            svg_line("270", "80", "450", "140", "mesh-link"),
            svg_line("450", "140", "620", "70", "mesh-link"),
            svg_line("450", "140", "690", "180", "mesh-link"),
            svg_line("110", "170", "450", "140", "mesh-link mesh-link-blocked"),
            svg_circle("110", "170", "10", "mesh-site"),
            svg_circle("270", "80", "10", "mesh-site"),
            svg_circle("450", "140", "10", "mesh-site"),
            svg_circle("620", "70", "10", "mesh-site"),
            svg_circle("690", "180", "10", "mesh-site"),
            svg_text("110", "200", "mesh-label", "fire station"),
            svg_text("270", "60", "mesh-label", "church steeple"),
            svg_text("450", "170", "mesh-label", "water tower"),
            svg_text("620", "50", "mesh-label", "ridgeline"),
            svg_text("690", "210", "mesh-label", "county garage"),
            element(
                "text",
                &[("x", "278"), ("y", "132"), ("class", "mesh-note")],
                vec![txt("direct path blocked, message hops around")],
            ),
        ],
    )
}

fn mesh_diagram_mobile() -> SiteView {
    element(
        "svg",
        &[
            ("class", "mesh-diagram mesh-diagram-mobile"),
            ("viewBox", "0 0 300 480"),
            ("role", "img"),
            (
                "aria-labelledby",
                "mesh-mobile-title mesh-mobile-description",
            ),
        ],
        vec![
            element(
                "title",
                &[("id", "mesh-mobile-title")],
                vec![txt("A message relayed around a blocked direct path")],
            ),
            element(
                "desc",
                &[("id", "mesh-mobile-description")],
                vec![txt(
                    "Five radios form a zigzag relay path from a fire station through a church and water tower, then branch to a ridgeline and county garage.",
                )],
            ),
            svg_line("55", "420", "215", "350", "mesh-link"),
            svg_line("215", "350", "90", "250", "mesh-link"),
            svg_line("90", "250", "220", "140", "mesh-link"),
            svg_line("90", "250", "230", "300", "mesh-link"),
            svg_line("55", "420", "90", "250", "mesh-link mesh-link-blocked"),
            svg_circle("55", "420", "10", "mesh-site mesh-site-mobile"),
            svg_circle("215", "350", "10", "mesh-site mesh-site-mobile"),
            svg_circle("90", "250", "10", "mesh-site mesh-site-mobile"),
            svg_circle("220", "140", "10", "mesh-site mesh-site-mobile"),
            svg_circle("230", "300", "10", "mesh-site mesh-site-mobile"),
            svg_text("55", "450", "mesh-label", "fire station"),
            svg_text("215", "380", "mesh-label", "church steeple"),
            svg_text("90", "225", "mesh-label", "water tower"),
            svg_text("220", "115", "mesh-label", "ridgeline"),
            svg_text("230", "330", "mesh-label", "county garage"),
            element(
                "text",
                &[
                    ("x", "18"),
                    ("y", "335"),
                    ("class", "mesh-note mesh-note-mobile"),
                ],
                vec![txt("direct path blocked")],
            ),
        ],
    )
}

fn pilot() -> SiteView {
    element(
        "section",
        &[("class", "content-section")],
        vec![
            section_heading("04", "the FIVCO pilot"),
            element(
                "div",
                &[("class", "pilot-grid")],
                vec![
                    element(
                        "figure",
                        &[("class", "county-card")],
                        vec![
                            county_map(),
                            element(
                                "figcaption",
                                &[],
                                vec![txt("ten proposed sites · stylized, not to scale")],
                            ),
                        ],
                    ),
                    element(
                        "div",
                        &[("class", "pilot-copy")],
                        vec![
                            element(
                                "p",
                                &[],
                                vec![txt(
                                    "Ten sites across Boyd, Carter, Elliott, Greenup, and Lawrence counties, hosted by local organizations and public facilities, with high ground prioritized for useful range.",
                                )],
                            ),
                            element(
                                "p",
                                &[("class", "list-lead")],
                                vec![txt("The pilot produces three things:")],
                            ),
                            element(
                                "ol",
                                &[("class", "deliverable-list")],
                                vec![
                                    element(
                                        "li",
                                        &[],
                                        vec![txt(
                                            "A working backup messaging layer for participating communities.",
                                        )],
                                    ),
                                    element("li", &[], vec![txt("A measured coverage map.")]),
                                    element(
                                        "li",
                                        &[],
                                        vec![txt(
                                            "A costed, step-by-step playbook other Appalachian counties can copy.",
                                        )],
                                    ),
                                ],
                            ),
                            element(
                                "aside",
                                &[("class", "callout")],
                                vec![txt(
                                    "Three working radios, built in a single day, are exchanging data over the air now. A county-scale pilot is chiefly a materials, siting, and training problem.",
                                )],
                            ),
                        ],
                    ),
                ],
            ),
        ],
    )
}

fn county_map() -> SiteView {
    element(
        "svg",
        &[
            ("class", "county-map"),
            ("viewBox", "0 0 300 320"),
            ("role", "img"),
            ("aria-labelledby", "county-map-title county-map-description"),
        ],
        vec![
            element(
                "title",
                &[("id", "county-map-title")],
                vec![txt("FIVCO pilot network")],
            ),
            element(
                "desc",
                &[("id", "county-map-description")],
                vec![txt(
                    "A stylized map of Boyd, Carter, Elliott, Greenup, and Lawrence counties connected by ten proposed radio sites.",
                )],
            ),
            county_shape(
                "M60 18 L150 10 L172 52 L150 108 L74 116 L48 60 Z",
                "county-shape county-shape-sage",
            ),
            county_shape("M150 10 L236 24 L252 96 L172 52 Z", "county-shape"),
            county_shape("M172 52 L252 96 L244 178 L150 108 Z", "county-shape"),
            county_shape(
                "M74 116 L150 108 L244 178 L200 260 L96 244 Z",
                "county-shape county-shape-sage",
            ),
            county_shape("M96 244 L200 260 L212 308 L108 312 Z", "county-shape"),
            svg_text("104", "66", "county-label county-label-sage", "GREENUP"),
            svg_text("204", "52", "county-label", "BOYD"),
            svg_text("206", "122", "county-label", "CARTER"),
            svg_text("152", "196", "county-label county-label-sage", "ELLIOTT"),
            svg_text("158", "290", "county-label", "LAWRENCE"),
            svg_line("110", "44", "206", "36", "county-link"),
            svg_line("206", "36", "216", "100", "county-link"),
            svg_line("110", "44", "126", "90", "county-link"),
            svg_line("126", "90", "216", "100", "county-link"),
            svg_line("126", "90", "140", "170", "county-link"),
            svg_line("216", "100", "196", "150", "county-link"),
            svg_line("140", "170", "196", "150", "county-link"),
            svg_line("140", "170", "128", "228", "county-link"),
            svg_line("128", "228", "176", "276", "county-link"),
            svg_line("196", "150", "176", "276", "county-link"),
            svg_line("90", "140", "126", "90", "county-link"),
            svg_line("232", "210", "196", "150", "county-link"),
            svg_circle("110", "44", "6", "county-site"),
            svg_circle("206", "36", "6", "county-site"),
            svg_circle("216", "100", "6", "county-site"),
            svg_circle("126", "90", "6", "county-site"),
            svg_circle("140", "170", "6", "county-site"),
            svg_circle("196", "150", "6", "county-site"),
            svg_circle("128", "228", "6", "county-site"),
            svg_circle("176", "276", "6", "county-site"),
            svg_circle("90", "140", "6", "county-site"),
            svg_circle("232", "210", "6", "county-site"),
        ],
    )
}

fn county_shape(path: &str, class: &str) -> SiteView {
    element("path", &[("d", path), ("class", class)], vec![])
}

fn svg_line(x1: &str, y1: &str, x2: &str, y2: &str, class: &str) -> SiteView {
    element(
        "line",
        &[
            ("x1", x1),
            ("y1", y1),
            ("x2", x2),
            ("y2", y2),
            ("class", class),
        ],
        vec![],
    )
}

fn svg_circle(cx: &str, cy: &str, radius: &str, class: &str) -> SiteView {
    element(
        "circle",
        &[("cx", cx), ("cy", cy), ("r", radius), ("class", class)],
        vec![],
    )
}

fn svg_text(x: &str, y: &str, class: &str, label: &str) -> SiteView {
    element(
        "text",
        &[
            ("x", x),
            ("y", y),
            ("text-anchor", "middle"),
            ("class", class),
        ],
        vec![txt(label)],
    )
}

fn costs() -> SiteView {
    element(
        "section",
        &[("class", "content-section")],
        vec![
            section_heading("05", "what it costs"),
            element(
                "div",
                &[("class", "table-wrap")],
                vec![element(
                    "table",
                    &[],
                    vec![
                        element(
                            "caption",
                            &[("class", "sr-only")],
                            vec![txt("Typical community radio hardware costs")],
                        ),
                        element(
                            "thead",
                            &[],
                            vec![element(
                                "tr",
                                &[],
                                vec![
                                    element("th", &[("scope", "col")], vec![txt("Item")]),
                                    element("th", &[("scope", "col")], vec![txt("Estimate")]),
                                ],
                            )],
                        ),
                        element(
                            "tbody",
                            &[],
                            vec![
                                cost("Heltec V4 radio, assembled and programmed", "~ $50"),
                                cost("T114 radio", "~ $30"),
                                cost("All-in-one solar node", "~ $100"),
                                cost("Battery, solar panel, or wall power", "varies by site"),
                                cost("Monthly service fees or subscriptions", "none"),
                                cost(
                                    "Ten-site county pilot",
                                    "materials + installation + training",
                                ),
                            ],
                        ),
                    ],
                )],
            ),
            element(
                "p",
                &[("class", "aside-copy")],
                vec![txt(
                    "Exact site costs depend on the host and placement. Measuring them is part of the pilot.",
                )],
            ),
        ],
    )
}

fn cost(item: &str, estimate: &str) -> SiteView {
    element(
        "tr",
        &[],
        vec![
            element("th", &[("scope", "row")], vec![txt(item)]),
            element("td", &[], vec![txt(estimate)]),
        ],
    )
}

fn partnership() -> SiteView {
    element(
        "section",
        &[("class", "two-up closing-grid")],
        vec![
            element(
                "article",
                &[("class", "info-card")],
                vec![
                    element(
                        "p",
                        &[("class", "card-kicker")],
                        vec![txt("06 · partnership")],
                    ),
                    element(
                        "p",
                        &[],
                        vec![txt(
                            "Merely is the technical partner: building, installing, and maintaining equipment, then training local hosts. An eligible public or nonprofit partner holds grant funds. The community owns its network.",
                        )],
                    ),
                ],
            ),
            element(
                "article",
                &[("class", "night-card")],
                vec![
                    element(
                        "p",
                        &[("class", "card-kicker")],
                        vec![txt("07 · open source")],
                    ),
                    element(
                        "p",
                        &[],
                        vec![txt(
                            "Retinue is the open-source radio stack. It supports the major mesh networking standards today while we work toward a simple, secure way to manage your own radios.",
                        )],
                    ),
                    external_link(
                        "https://github.com/merely-made/retinue",
                        "Read the Retinue source ↗",
                        "button button-night",
                    ),
                ],
            ),
        ],
    )
}
