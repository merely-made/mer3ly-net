use genet_scripted_dom::ScriptedDom;
use layout_dom_api::LayoutDom;
use mer3ly_site::pages::{home, radio};
use mer3ly_site::site::SITE_CSS;

const FORBIDDEN_PUBLIC_MARKERS: &[&str] = &[
    "mailto:",
    "tel:",
    "outlook.com",
    "C:\\Users\\",
    "support.js",
    "<script src",
    "x-dc",
    "__next",
    "webpack",
];

#[test]
fn pages_are_static_genet_documents() {
    for (name, document) in [("home", home::document()), ("radio", radio::document())] {
        assert!(
            document.starts_with("<!doctype html>"),
            "{name} has an HTML doctype"
        );
        assert_eq!(
            document.matches("<h1").count(),
            1,
            "{name} has one primary heading"
        );
        assert!(
            document.contains("<main id=\"main\""),
            "{name} exposes the skip-link target"
        );
        assert!(
            document.contains("<link rel=\"canonical\""),
            "{name} has a canonical URL"
        );
        assert!(
            document.contains("application/ld+json"),
            "{name} has structured data"
        );
        assert!(
            document.contains("https://mer3ly.net/og.jpg"),
            "{name} names the generated social preview"
        );

        for marker in FORBIDDEN_PUBLIC_MARKERS {
            assert!(
                !document.contains(marker),
                "{name} contains forbidden public marker {marker:?}"
            );
        }

        let dom = ScriptedDom::from_serialized_document(&document);
        let serialized = dom.inner_html(dom.document());
        assert!(
            serialized.contains("<html lang=\"en\">"),
            "Genet parses and serializes the {name} document"
        );
        assert!(
            serialized.contains("<footer class=\"site-footer\">"),
            "Genet preserves the {name} landmarks"
        );
    }
}

#[test]
fn static_baseline_stays_below_the_m3_budget() {
    let bytes = home::document().len() + radio::document().len() + SITE_CSS.len();
    assert!(
        bytes < 200 * 1024,
        "base HTML and CSS use {bytes} bytes, over the 200 KiB M3 budget"
    );
}

#[test]
fn stylesheet_has_responsive_and_accessibility_contracts() {
    for contract in [
        "@media (max-width: 760px)",
        "@media (max-width: 440px)",
        "@media (prefers-reduced-motion: reduce)",
        ".skip-link",
        ":focus-visible",
    ] {
        assert!(
            SITE_CSS.contains(contract),
            "site CSS is missing {contract}"
        );
    }
}
