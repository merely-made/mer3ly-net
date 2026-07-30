use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use mer3ly_site::pages::repositories;
use mer3ly_site::repositories::PublicSiteData;
use mer3ly_site::site::SITE_CSS;
use serde::Deserialize;

const GRAPH_LOADER: &str = include_str!("../assets/repo-graph.js");
const GRAPH_GLUE: &str = include_str!("../assets/mer3ly_repo_graph.js");
const GRAPH_WASM: &[u8] = include_bytes!("../assets/mer3ly_repo_graph_bg.wasm");

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

#[derive(Deserialize)]
struct GraphAuthority {
    schema: String,
    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,
}

#[derive(Deserialize)]
struct GraphNode {
    id: String,
}

#[derive(Deserialize)]
struct GraphEdge {
    id: String,
    source: String,
    target: String,
}

fn graph_authority(document: &str) -> GraphAuthority {
    let marker = "<script id=\"repository-graph-data\" type=\"application/json\">";
    let start = document.find(marker).expect("repository graph bootstrap") + marker.len();
    let end = document[start..]
        .find("</script>")
        .map(|offset| start + offset)
        .expect("repository graph bootstrap end");
    serde_json::from_str(&document[start..end]).expect("valid graph authority JSON")
}

#[test]
fn graph_and_semantic_index_share_exact_public_ids() {
    let root = workspace_root();
    let data = PublicSiteData::load(&root).expect("load validated public site data");
    let document = repositories::document(&root).expect("render repository page");
    let graph = graph_authority(&document);

    assert_eq!(graph.schema, "mer3ly.repo-graph/v1");
    assert_eq!(
        graph
            .nodes
            .iter()
            .map(|node| node.id.as_str())
            .collect::<BTreeSet<_>>(),
        data.authority
            .repositories
            .repository
            .iter()
            .filter(|repository| repository.public)
            .map(|repository| repository.id.as_str())
            .collect::<BTreeSet<_>>()
    );
    assert_eq!(
        graph
            .edges
            .iter()
            .map(|edge| edge.id.as_str())
            .collect::<BTreeSet<_>>(),
        data.authority
            .relations
            .relation
            .iter()
            .map(|relation| relation.id.as_str())
            .collect::<BTreeSet<_>>()
    );

    let node_ids = graph
        .nodes
        .iter()
        .map(|node| node.id.as_str())
        .collect::<BTreeSet<_>>();
    for edge in &graph.edges {
        assert!(node_ids.contains(edge.source.as_str()));
        assert!(node_ids.contains(edge.target.as_str()));
    }
    for repository in &data.authority.repositories.repository {
        assert!(document.contains(&format!("id=\"repo-{}\"", repository.id)));
        assert!(document.contains(&format!("data-repository-id=\"{}\"", repository.id)));
    }
}

#[test]
fn graph_enhancement_preserves_the_visible_static_fallback() {
    let root = workspace_root();
    let data = PublicSiteData::load(&root).expect("load validated public site data");
    let document = repositories::document(&root).expect("render repository page");
    let fallback = document
        .find("data-graph-fallback")
        .expect("visible graph fallback");
    let interface = document
        .find("data-graph-interface")
        .expect("hidden graph interface");
    let index = document
        .find("class=\"content-section repository-index\"")
        .expect("semantic repository index");

    assert!(fallback < interface);
    assert!(interface < index);
    assert!(document[interface..].contains("hidden=\"hidden\""));
    assert_eq!(
        document.match_indices("data-repository-id=").count(),
        data.authority.repositories.repository.len()
    );
    assert!(document.contains("The complete repository index remains available below."));
}

#[test]
fn graph_runtime_covers_interaction_and_failure_contracts() {
    for contract in [
        "navigator.gpu",
        "layoutGraph(JSON.stringify(authority))",
        "dataset.graphState",
        "visibilitychange",
        "requestAnimationFrame",
        "prefers-reduced-motion: reduce",
        "aria-pressed",
        "ArrowRight",
        "Home",
        "Enter",
        "pointerdown",
        "\"wheel\"",
        "window.location.assign",
        "dataset.projectHref",
        "no-webgpu",
        "no-wasm",
        "init-failure",
        "\"motion\") === \"reduce\"",
    ] {
        assert!(
            GRAPH_LOADER.contains(contract),
            "graph loader is missing {contract}"
        );
    }

    for forbidden in [
        "Graphshell",
        "Personae",
        "browser history",
        "resident host",
        "C:\\Users\\",
        "mark_",
    ] {
        assert!(
            !GRAPH_LOADER.contains(forbidden)
                && !GRAPH_GLUE.contains(forbidden)
                && !String::from_utf8_lossy(GRAPH_WASM).contains(forbidden),
            "graph runtime contains forbidden marker {forbidden:?}"
        );
    }
}

#[test]
fn graph_assets_and_responsive_styles_are_bounded() {
    assert_eq!(&GRAPH_WASM[..4], b"\0asm");
    assert!(
        GRAPH_WASM.len() < 256 * 1024,
        "graph Wasm is {} bytes",
        GRAPH_WASM.len()
    );
    assert!(
        GRAPH_LOADER.len() + GRAPH_GLUE.len() + GRAPH_WASM.len() < 320 * 1024,
        "graph runtime is {} bytes",
        GRAPH_LOADER.len() + GRAPH_GLUE.len() + GRAPH_WASM.len()
    );

    for contract in [
        ".repository-graph-interface[hidden]",
        ".repository-graph-node:focus-visible",
        ".repository-graph-node.is-selected",
        "@media (max-width: 760px)",
        "@media (max-width: 440px)",
        "@media (prefers-reduced-motion: reduce)",
        ".repository-graph-section",
    ] {
        assert!(
            SITE_CSS.contains(contract),
            "site CSS is missing {contract}"
        );
    }
}
