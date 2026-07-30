use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use genet_scripted_dom::ScriptedDom;
use layout_dom_api::LayoutDom;
use mer3ly_site::pages::{home, projects};
use mer3ly_site::repositories::PublicSiteData;
use mer3ly_site::site::SITE_CSS;

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

#[test]
fn showcase_authority_is_bounded_and_ordered() {
    let root = workspace_root();
    let data = PublicSiteData::load(&root).expect("load validated public site data");
    let showcases = data.showcases.ordered();

    assert_eq!(showcases.len(), 5);
    assert_eq!(
        showcases
            .iter()
            .map(|showcase| showcase.repository.as_str())
            .collect::<Vec<_>>(),
        ["mere", "genet", "turnstone", "woodshed", "isometry"]
    );
    for showcase in showcases {
        assert_eq!(
            showcase.image,
            format!("showcase/{}.png", showcase.repository)
        );
        assert!(
            showcase
                .source_url
                .starts_with("https://github.com/merely-made/")
        );
        assert!(root.join("assets").join(&showcase.image).is_file());
    }
}

#[test]
fn home_projects_every_showcase_into_a_local_profile() {
    let root = workspace_root();
    let data = PublicSiteData::load(&root).expect("load validated public site data");
    let document = home::document_for(&data);

    for showcase in data.showcases.ordered() {
        assert!(document.contains(&format!("src=\"/{}\"", showcase.image)));
        assert!(document.contains(&format!("href=\"/projects/{}/\"", showcase.repository)));
        assert!(document.contains(&showcase.headline));
    }
    assert!(document.contains("class=\"home-showcase-list\""));
    assert_eq!(document.matches("<h1").count(), 1);

    let dom = ScriptedDom::from_serialized_document(&document);
    let serialized = dom.inner_html(dom.document());
    assert!(serialized.contains("class=\"home-showcase-card\""));
}

#[test]
fn every_public_repository_has_one_semantic_project_profile() {
    let root = workspace_root();
    let data = PublicSiteData::load(&root).expect("load validated public site data");
    let documents = projects::documents(&data);
    let expected_repositories = data
        .authority
        .repositories
        .repository
        .iter()
        .filter(|repository| repository.public)
        .count();
    assert_eq!(documents.len(), expected_repositories);

    let mut relation_counts = BTreeMap::new();
    for (repository_id, document) in &documents {
        assert!(document.starts_with("<!doctype html>"));
        assert_eq!(document.matches("<h1").count(), 1);
        assert!(document.contains(&format!("data-project-id=\"{repository_id}\"")));
        assert!(document.contains(&format!("https://mer3ly.net/projects/{repository_id}/")));
        assert!(document.contains("href=\"mailto:markik@mer3ly.net\""));
        for relation in &data.authority.relations.relation {
            *relation_counts
                .entry(relation.id.as_str())
                .or_insert(0_usize) += document
                .matches(&format!("data-relation-id=\"{}\"", relation.id))
                .count();
        }
    }

    for relation in &data.authority.relations.relation {
        assert_eq!(
            relation_counts.get(relation.id.as_str()),
            Some(&2),
            "relation {} appears once on each endpoint profile",
            relation.id
        );
    }
}

#[test]
fn visual_and_text_only_profiles_state_their_evidence_boundary() {
    let root = workspace_root();
    let mere = projects::document(&root, "mere").expect("render Mere profile");
    let retinue = projects::document(&root, "retinue").expect("render Retinue profile");

    assert!(mere.contains("src=\"/showcase/mere.png\""));
    assert!(mere.contains("Source image:"));
    assert!(mere.contains("License: MIT OR Apache-2.0."));
    assert!(retinue.contains("This profile is intentionally text-first."));
    assert!(!retinue.contains("project-showcase-figure"));
}

#[test]
fn showcase_styles_cover_responsive_images_and_profile_relations() {
    for contract in [
        ".home-showcase-card",
        "object-fit: contain",
        ".project-showcase-layout",
        ".project-relation-columns",
        ".project-facts-layout",
        ".project-profile-hero",
        "@media (max-width: 760px)",
        "@media (max-width: 440px)",
    ] {
        assert!(
            SITE_CSS.contains(contract),
            "site CSS is missing {contract}"
        );
    }
}
