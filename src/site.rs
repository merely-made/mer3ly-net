use std::cell::RefCell;
use std::rc::Rc;

use cambium::{AnyView, GenetAppRunner, GenetCtx, GenetElement, el, text};
use genet_scripted_dom::ScriptedDom;

pub type SiteView = Box<dyn AnyView<(), (), GenetCtx, GenetElement>>;

pub const SITE_CSS: &str = include_str!("../assets/site.css");

const ORGANIZATION_JSON_LD: &str = r#"{
  "@context": "https://schema.org",
  "@type": "Organization",
  "name": "Merely LLC",
  "url": "https://mer3ly.net/",
  "sameAs": ["https://github.com/merely-made"],
  "address": {
    "@type": "PostalAddress",
    "addressLocality": "Ashland",
    "addressRegion": "KY",
    "addressCountry": "US"
  }
}"#;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActivePage {
    Home,
    Radio,
    Repositories,
}

pub struct PageMetadata {
    pub title: &'static str,
    pub description: &'static str,
    pub canonical_url: &'static str,
}

pub fn txt(value: impl Into<String>) -> SiteView {
    Box::new(text(value))
}

pub fn element(tag: &str, attrs: &[(&str, &str)], children: Vec<SiteView>) -> SiteView {
    let mut node = el::<_, (), ()>(tag, children);
    for (name, value) in attrs {
        node = node.attr(*name, *value);
    }
    Box::new(node)
}

pub fn link(href: &str, label: impl Into<String>, class: &str) -> SiteView {
    element("a", &[("href", href), ("class", class)], vec![txt(label)])
}

pub fn external_link(href: &str, label: impl Into<String>, class: &str) -> SiteView {
    element(
        "a",
        &[("href", href), ("class", class), ("rel", "noreferrer")],
        vec![txt(label)],
    )
}

pub fn section_heading(number: &str, title: &str) -> SiteView {
    element(
        "div",
        &[("class", "section-heading")],
        vec![
            element(
                "span",
                &[("class", "section-number"), ("aria-hidden", "true")],
                vec![txt(number)],
            ),
            element("h2", &[], vec![txt(title)]),
        ],
    )
}

pub fn shell(active: ActivePage, main: SiteView) -> SiteView {
    let home_attrs = if active == ActivePage::Home {
        vec![
            ("href", "/"),
            ("aria-current", "page"),
            ("class", "nav-link is-current"),
        ]
    } else {
        vec![("href", "/"), ("class", "nav-link")]
    };
    let radio_attrs = if active == ActivePage::Radio {
        vec![
            ("href", "/radio.html"),
            ("aria-current", "page"),
            ("class", "nav-link is-current"),
        ]
    } else {
        vec![("href", "/radio.html"), ("class", "nav-link")]
    };
    let repositories_attrs = if active == ActivePage::Repositories {
        vec![
            ("href", "/repos/"),
            ("aria-current", "page"),
            ("class", "nav-link is-current"),
        ]
    } else {
        vec![("href", "/repos/"), ("class", "nav-link")]
    };

    let header = element(
        "header",
        &[("class", "site-header")],
        vec![
            element(
                "a",
                &[("href", "/"), ("class", "brand")],
                vec![
                    element("span", &[("class", "brand-name")], vec![txt("merely")]),
                    element(
                        "span",
                        &[("class", "brand-kind")],
                        vec![txt("software + hardware")],
                    ),
                ],
            ),
            element(
                "nav",
                &[("aria-label", "Main navigation")],
                vec![element(
                    "ul",
                    &[("class", "nav-list")],
                    vec![
                        element(
                            "li",
                            &[],
                            vec![element("a", &home_attrs, vec![txt("home")])],
                        ),
                        element(
                            "li",
                            &[],
                            vec![element("a", &repositories_attrs, vec![txt("repositories")])],
                        ),
                        element(
                            "li",
                            &[],
                            vec![element("a", &radio_attrs, vec![txt("community radio")])],
                        ),
                        element(
                            "li",
                            &[],
                            vec![external_link(
                                "https://github.com/merely-made",
                                "github ↗",
                                "nav-link",
                            )],
                        ),
                    ],
                )],
            ),
        ],
    );

    let footer = element(
        "footer",
        &[("class", "site-footer")],
        vec![
            element(
                "p",
                &[],
                vec![txt("Merely LLC · Ashland, Kentucky · mer3ly.net")],
            ),
            element(
                "p",
                &[],
                vec![external_link(
                    "https://github.com/merely-made",
                    "Public work and contact on GitHub ↗",
                    "footer-link",
                )],
            ),
        ],
    );

    element(
        "body",
        &[("class", "site-body")],
        vec![
            link("#main", "Skip to content", "skip-link"),
            element(
                "div",
                &[("class", "page-shell")],
                vec![header, main, footer],
            ),
        ],
    )
}

pub fn render_with(metadata: &PageMetadata, view: impl Fn() -> SiteView) -> String {
    let dom = Rc::new(RefCell::new(ScriptedDom::new()));
    let runner = GenetAppRunner::<_, _, _, ()>::new(dom, move |_: &()| view(), ());
    let body_markup = runner.dom().borrow().outer_html(runner.root());
    render_shell(metadata, &body_markup)
}

fn render_shell(metadata: &PageMetadata, body_markup: &str) -> String {
    format!(
        "<!doctype html>\n\
<html lang=\"en\">\n\
<head>\n\
  <meta charset=\"utf-8\">\n\
  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
  <title>{title}</title>\n\
  <meta name=\"description\" content=\"{description}\">\n\
  <link rel=\"canonical\" href=\"{canonical}\">\n\
  <meta property=\"og:type\" content=\"website\">\n\
  <meta property=\"og:site_name\" content=\"Merely\">\n\
  <meta property=\"og:title\" content=\"{title}\">\n\
  <meta property=\"og:description\" content=\"{description}\">\n\
  <meta property=\"og:url\" content=\"{canonical}\">\n\
  <meta property=\"og:image\" content=\"https://mer3ly.net/og.jpg\">\n\
  <meta name=\"twitter:card\" content=\"summary_large_image\">\n\
  <meta name=\"theme-color\" content=\"#f0ebdd\">\n\
  <link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">\n\
  <link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>\n\
  <link rel=\"stylesheet\" href=\"https://fonts.googleapis.com/css2?family=Young+Serif&family=Newsreader:ital,opsz,wght@0,6..72,400;0,6..72,500;0,6..72,600;1,6..72,400&family=IBM+Plex+Mono:wght@400;500;600&display=swap\">\n\
  <link rel=\"stylesheet\" href=\"/site.css\">\n\
  <script type=\"application/ld+json\">{json_ld}</script>\n\
</head>\n\
{body}\n\
</html>\n",
        title = escape_text(metadata.title),
        description = escape_attr(metadata.description),
        canonical = escape_attr(metadata.canonical_url),
        json_ld = ORGANIZATION_JSON_LD,
        body = body_markup,
    )
}

fn escape_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_attr(value: &str) -> String {
    escape_text(value).replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_escaping_is_safe() {
        assert_eq!(escape_text("A & B < C"), "A &amp; B &lt; C");
        assert_eq!(escape_attr("\"quoted\""), "&quot;quoted&quot;");
    }
}
