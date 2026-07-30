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
                    element(
                        "ol",
                        &[
                            ("class", "mesh-route"),
                            (
                                "aria-label",
                                "A message hops from a fire station through community relay sites",
                            ),
                        ],
                        vec![
                            relay("fire station", "origin"),
                            relay("church steeple", "relay"),
                            relay("water tower", "relay"),
                            relay("ridgeline", "relay"),
                            relay("county garage", "destination"),
                        ],
                    ),
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

fn relay(label: &str, role: &str) -> SiteView {
    element(
        "li",
        &[("class", "relay-node")],
        vec![
            element(
                "span",
                &[("class", "relay-dot"), ("aria-hidden", "true")],
                vec![],
            ),
            element("span", &[("class", "relay-label")], vec![txt(label)]),
            element("span", &[("class", "sr-only")], vec![txt(role)]),
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
                            element(
                                "ul",
                                &[
                                    ("class", "county-grid"),
                                    ("aria-label", "Five FIVCO counties"),
                                ],
                                vec![
                                    county("Greenup", "01"),
                                    county("Boyd", "02"),
                                    county("Carter", "03"),
                                    county("Elliott", "04"),
                                    county("Lawrence", "05"),
                                ],
                            ),
                            element(
                                "figcaption",
                                &[],
                                vec![txt("five counties · ten proposed sites")],
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

fn county(name: &str, number: &str) -> SiteView {
    element(
        "li",
        &[],
        vec![
            element("span", &[("class", "county-number")], vec![txt(number)]),
            element("span", &[], vec![txt(name)]),
        ],
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
