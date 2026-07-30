use crate::repositories::{Authority, PublicSiteData};

pub const ROBOTS_TXT: &str = "User-agent: *\nAllow: /\nSitemap: https://mer3ly.net/sitemap.xml\n";

pub fn canonical_urls(data: &PublicSiteData) -> Vec<String> {
    canonical_urls_from_authority(&data.authority)
}

pub fn canonical_urls_from_authority(authority: &Authority) -> Vec<String> {
    let mut urls = vec![
        "https://mer3ly.net/".to_owned(),
        "https://mer3ly.net/repos/".to_owned(),
        "https://mer3ly.net/radio.html".to_owned(),
    ];
    urls.extend(
        authority
            .repositories
            .repository
            .iter()
            .filter(|repository| repository.public)
            .map(|repository| format!("https://mer3ly.net/projects/{}/", repository.id)),
    );
    urls
}

pub fn sitemap(data: &PublicSiteData) -> String {
    let entries = canonical_urls(data)
        .into_iter()
        .map(|url| format!("  <url><loc>{}</loc></url>", escape_xml(&url)))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n\
{entries}\n\
</urlset>\n"
    )
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('\'', "&apos;")
        .replace('"', "&quot;")
        .replace('>', "&gt;")
        .replace('<', "&lt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xml_escaping_covers_sitemap_entities() {
        assert_eq!(
            escape_xml("https://example.com/?a=1&b=\"two\""),
            "https://example.com/?a=1&amp;b=&quot;two&quot;"
        );
    }
}
