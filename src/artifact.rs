use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::net::IpAddr;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::repositories::{Authority, PublicMetadataCache};

const RECEIPT_SCHEMA: &str = "mer3ly.public-artifact-receipt/v1";
const GRAPH_SCHEMA: &str = "mer3ly.repo-graph/v1";
const EXPECTED_FILES: &[&str] = &[
    "CNAME",
    "index.html",
    "mer3ly_repo_graph.js",
    "mer3ly_repo_graph_bg.wasm",
    "og.jpg",
    "radio.html",
    "repo-graph.js",
    "repos/index.html",
    "site.css",
];

#[derive(Debug, Serialize)]
pub struct ArtifactReceipt {
    schema: &'static str,
    source_sha: String,
    files: Vec<ArtifactFileReceipt>,
    total_bytes: u64,
    repositories: usize,
    relation_text_projections: usize,
    graph_nodes: usize,
    graph_edges: usize,
    metadata_generated_at_utc: String,
    metadata_sha256: String,
}

#[derive(Debug, Serialize)]
struct ArtifactFileReceipt {
    path: String,
    bytes: u64,
    sha256: String,
}

#[derive(Debug, serde::Deserialize)]
struct GraphPayload {
    schema: String,
    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,
}

#[derive(Debug, serde::Deserialize)]
struct GraphNode {
    id: String,
}

#[derive(Debug, serde::Deserialize)]
struct GraphEdge {
    id: String,
    source: String,
    target: String,
}

pub fn validate_public_artifact(
    artifact_root: &Path,
    authority: &Authority,
    metadata: &PublicMetadataCache,
    metadata_path: &Path,
) -> Result<ArtifactReceipt, Vec<String>> {
    let mut errors = Vec::new();
    let mut files = Vec::new();
    if let Err(error) = collect_files(artifact_root, artifact_root, &mut files) {
        return Err(vec![format!("could not read public artifact: {error}")]);
    }
    files.sort();

    let actual_paths = files
        .iter()
        .map(|path| artifact_relative_path(artifact_root, path))
        .collect::<BTreeSet<_>>();
    let expected_paths = EXPECTED_FILES
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();
    if actual_paths != expected_paths {
        errors.push("public artifact file set differs from the approved shape".to_owned());
    }

    let allowed_github_slugs = authority
        .repositories
        .repository
        .iter()
        .filter(|repository| repository.public)
        .map(|repository| repository.github_slug.to_ascii_lowercase())
        .collect::<BTreeSet<_>>();

    let mut file_receipts = Vec::new();
    let mut total_bytes = 0_u64;
    for path in &files {
        let relative = artifact_relative_path(artifact_root, path);
        let bytes = match fs::read(path) {
            Ok(bytes) => bytes,
            Err(error) => {
                errors.push(format!(
                    "could not read public artifact file {relative}: {error}"
                ));
                continue;
            }
        };
        total_bytes = total_bytes.saturating_add(bytes.len() as u64);
        if is_scannable(&relative) {
            scan_public_text(
                &relative,
                &String::from_utf8_lossy(&bytes),
                &allowed_github_slugs,
                &mut errors,
            );
        }
        file_receipts.push(ArtifactFileReceipt {
            path: relative,
            bytes: bytes.len() as u64,
            sha256: sha256(&bytes),
        });
    }

    validate_cname(artifact_root, "CNAME", &mut errors);

    let home = read_text(artifact_root, "index.html", &mut errors);
    let radio = read_text(artifact_root, "radio.html", &mut errors);
    let repositories = read_text(artifact_root, "repos/index.html", &mut errors);
    if !home.starts_with("<!doctype html>") || !radio.starts_with("<!doctype html>") {
        errors.push("home or community-radio output is not a complete HTML document".to_owned());
    }

    let repository_ids = attribute_values(&repositories, "data-repository-id");
    let relation_ids = attribute_values(&repositories, "data-relation-id");
    validate_static_authority(&repository_ids, &relation_ids, authority, &mut errors);
    let repository_count = repository_ids.len();
    let relation_text_projections = relation_ids.len();
    if !repositories.contains("<script type=\"module\" src=\"/repo-graph.js\"></script>") {
        errors.push("repository page is missing the optional graph module".to_owned());
    }
    let timestamp = format!(
        "{} {} UTC",
        &metadata.generated_at_utc[..10],
        &metadata.generated_at_utc[11..16]
    );
    if !repositories.contains(&timestamp) {
        errors.push("repository page does not display the validated metadata timestamp".to_owned());
    }

    let graph = parse_graph_payload(&repositories, &mut errors);
    let graph_nodes = graph.as_ref().map_or(0, |payload| payload.nodes.len());
    let graph_edges = graph.as_ref().map_or(0, |payload| payload.edges.len());
    if let Some(payload) = &graph {
        validate_graph_payload(payload, authority, &mut errors);
    }

    let wasm_path = artifact_root.join("mer3ly_repo_graph_bg.wasm");
    match fs::read(&wasm_path) {
        Ok(bytes) if bytes.starts_with(b"\0asm") => {}
        Ok(_) => errors.push("repository graph Wasm has an invalid magic header".to_owned()),
        Err(error) => errors.push(format!("could not read repository graph Wasm: {error}")),
    }

    let metadata_bytes = match fs::read(metadata_path) {
        Ok(bytes) => bytes,
        Err(error) => {
            errors.push(format!("could not hash public metadata input: {error}"));
            Vec::new()
        }
    };

    if errors.is_empty() {
        Ok(ArtifactReceipt {
            schema: RECEIPT_SCHEMA,
            source_sha: env::var("GITHUB_SHA").unwrap_or_else(|_| "local".to_owned()),
            files: file_receipts,
            total_bytes,
            repositories: repository_count,
            relation_text_projections,
            graph_nodes,
            graph_edges,
            metadata_generated_at_utc: metadata.generated_at_utc.clone(),
            metadata_sha256: sha256(&metadata_bytes),
        })
    } else {
        Err(errors)
    }
}

fn collect_files(root: &Path, current: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_symlink() {
            return Err(std::io::Error::other(format!(
                "symbolic links are not allowed in the public artifact: {}",
                artifact_relative_path(root, &path)
            )));
        }
        if file_type.is_dir() {
            collect_files(root, &path, files)?;
        } else if file_type.is_file() {
            files.push(path);
        }
    }
    Ok(())
}

fn artifact_relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn is_scannable(path: &str) -> bool {
    path.ends_with(".html")
        || path.ends_with(".css")
        || path.ends_with(".js")
        || path.ends_with(".wasm")
        || path.ends_with("CNAME")
}

fn scan_public_text(
    relative_path: &str,
    text: &str,
    allowed_github_slugs: &BTreeSet<String>,
    errors: &mut Vec<String>,
) {
    let lower = text.to_ascii_lowercase();
    for marker in [
        "c:\\users\\",
        "\\users\\",
        "/users/",
        "/home/",
        "file://",
        "mark_",
        "viewerpermission",
        "\"viewer_permission\"",
        "\"ssh_url\"",
        "\"sshurl\"",
        "authorization: bearer",
        "github_pat_",
        "begin rsa private key",
        "begin ec private key",
        "begin openssh private key",
    ] {
        if lower.contains(marker) {
            errors.push(format!(
                "{relative_path} contains a forbidden public-data marker"
            ));
            break;
        }
    }

    let drive_path = Regex::new(
        r#"(?i)(?:^|[^a-z0-9])(?:[a-z]:[\\/](?:users|home|documents and settings|workspaces?|code)[\\/]|\\\\[a-z0-9._-]+[\\/])"#,
    )
    .expect("valid path regex");
    if drive_path.is_match(text) {
        errors.push(format!(
            "{relative_path} contains an absolute or network filesystem path"
        ));
    }

    let email =
        Regex::new(r"(?i)\b[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}\b").expect("valid email regex");
    if email.is_match(text) {
        errors.push(format!(
            "{relative_path} contains an unapproved contact address"
        ));
    }

    let private_host = Regex::new(r"(?i)(?:https?|wss?)://[a-z0-9.-]+\.(?:internal|local)\b")
        .expect("valid host regex");
    if private_host.is_match(text) {
        errors.push(format!("{relative_path} contains a private hostname"));
    }

    let ipv4 = Regex::new(r"\b(?:[0-9]{1,3}\.){3}[0-9]{1,3}\b").expect("valid IPv4 regex");
    if ipv4.find_iter(text).any(|candidate| {
        candidate
            .as_str()
            .parse::<IpAddr>()
            .is_ok_and(|address| match address {
                IpAddr::V4(address) => {
                    address.is_private()
                        || address.is_loopback()
                        || address.is_link_local()
                        || address.is_unspecified()
                }
                IpAddr::V6(_) => false,
            })
    }) {
        errors.push(format!(
            "{relative_path} contains a private or local network address"
        ));
    }

    let github = Regex::new(r"(?i)https://github\.com/([a-z0-9_.-]+)(?:/([a-z0-9_.-]+))?")
        .expect("valid GitHub URL regex");
    for captures in github.captures_iter(text) {
        let owner = captures
            .get(1)
            .map(|value| value.as_str().to_ascii_lowercase())
            .unwrap_or_default();
        let repository = captures
            .get(2)
            .map(|value| value.as_str().trim_end_matches(".git").to_ascii_lowercase());
        let approved = owner == "merely-made"
            && repository.as_ref().is_none_or(|repository| {
                allowed_github_slugs.contains(&format!("{owner}/{repository}"))
            });
        if !approved {
            errors.push(format!(
                "{relative_path} contains an unapproved GitHub repository link"
            ));
            break;
        }
    }
}

fn validate_cname(root: &Path, relative: &str, errors: &mut Vec<String>) {
    let contents = read_text(root, relative, errors);
    if contents.trim() != "mer3ly.net" {
        errors.push(format!(
            "{relative} does not name the approved public domain"
        ));
    }
}

fn read_text(root: &Path, relative: &str, errors: &mut Vec<String>) -> String {
    match fs::read_to_string(root.join(relative)) {
        Ok(contents) => contents,
        Err(error) => {
            errors.push(format!("could not read {relative}: {error}"));
            String::new()
        }
    }
}

fn parse_graph_payload(document: &str, errors: &mut Vec<String>) -> Option<GraphPayload> {
    let marker = "<script id=\"repository-graph-data\" type=\"application/json\">";
    let Some(start) = document.find(marker).map(|offset| offset + marker.len()) else {
        errors.push("repository page is missing graph authority data".to_owned());
        return None;
    };
    let Some(end) = document[start..]
        .find("</script>")
        .map(|offset| start + offset)
    else {
        errors.push("repository graph authority data is not terminated".to_owned());
        return None;
    };
    match serde_json::from_str(&document[start..end]) {
        Ok(payload) => Some(payload),
        Err(error) => {
            errors.push(format!(
                "repository graph authority data is invalid: {error}"
            ));
            None
        }
    }
}

fn attribute_values(document: &str, attribute: &str) -> Vec<String> {
    let pattern = format!(r#"{attribute}="([^"]+)""#);
    Regex::new(&pattern)
        .expect("valid generated attribute regex")
        .captures_iter(document)
        .filter_map(|captures| captures.get(1).map(|value| value.as_str().to_owned()))
        .collect()
}

fn validate_static_authority(
    repository_ids: &[String],
    relation_ids: &[String],
    authority: &Authority,
    errors: &mut Vec<String>,
) {
    let expected_repositories = authority
        .repositories
        .repository
        .iter()
        .filter(|repository| repository.public)
        .map(|repository| repository.id.as_str())
        .collect::<BTreeSet<_>>();
    let actual_repositories = repository_ids
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if repository_ids.len() != expected_repositories.len()
        || actual_repositories != expected_repositories
    {
        errors.push("public artifact repository ids differ from authority".to_owned());
    }

    let expected_relations = authority
        .relations
        .relation
        .iter()
        .map(|relation| (relation.id.as_str(), 2_usize))
        .collect::<BTreeMap<_, _>>();
    let mut actual_relations = BTreeMap::new();
    for relation_id in relation_ids {
        *actual_relations.entry(relation_id.as_str()).or_insert(0) += 1;
    }
    if actual_relations != expected_relations {
        errors.push("public artifact relation text ids differ from authority".to_owned());
    }
}

fn validate_graph_payload(payload: &GraphPayload, authority: &Authority, errors: &mut Vec<String>) {
    if payload.schema != GRAPH_SCHEMA {
        errors.push("repository graph authority schema is not approved".to_owned());
    }
    let expected_nodes = authority
        .repositories
        .repository
        .iter()
        .filter(|repository| repository.public)
        .map(|repository| repository.id.as_str())
        .collect::<BTreeSet<_>>();
    let actual_nodes = payload
        .nodes
        .iter()
        .map(|node| node.id.as_str())
        .collect::<BTreeSet<_>>();
    if actual_nodes != expected_nodes {
        errors.push("repository graph node ids differ from authority".to_owned());
    }

    let expected_edges = authority
        .relations
        .relation
        .iter()
        .map(|edge| edge.id.as_str())
        .collect::<BTreeSet<_>>();
    let actual_edges = payload
        .edges
        .iter()
        .map(|edge| edge.id.as_str())
        .collect::<BTreeSet<_>>();
    if actual_edges != expected_edges {
        errors.push("repository graph edge ids differ from authority".to_owned());
    }
    if payload.edges.iter().any(|edge| {
        !actual_nodes.contains(edge.source.as_str()) || !actual_nodes.contains(edge.target.as_str())
    }) {
        errors.push("repository graph contains an edge with an unknown endpoint".to_owned());
    }
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_text_scan_accepts_approved_github_links() {
        let allowed = BTreeSet::from(["merely-made/mere".to_owned()]);
        let mut errors = Vec::new();
        scan_public_text(
            "index.html",
            "https://github.com/merely-made https://github.com/merely-made/mere",
            &allowed,
            &mut errors,
        );
        assert!(errors.is_empty(), "{errors:?}");
    }

    #[test]
    fn public_text_scan_rejects_private_boundaries_without_echoing_values() {
        let allowed = BTreeSet::from(["merely-made/mere".to_owned()]);
        let mut errors = Vec::new();
        scan_public_text(
            "index.html",
            "C:\\Users\\person\\secret 192.168.1.4 person@example.com https://github.com/private-owner/private-repo",
            &allowed,
            &mut errors,
        );
        assert!(errors.len() >= 4);
        let joined = errors.join("\n");
        assert!(!joined.contains("person"));
        assert!(!joined.contains("192.168.1.4"));
        assert!(!joined.contains("private-owner"));
    }
}
