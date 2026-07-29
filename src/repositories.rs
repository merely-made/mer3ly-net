use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

const REPOSITORIES_PATH: &str = "content/repositories.toml";
const RELATIONS_PATH: &str = "content/relations.toml";
const MIGRATION_PATH: &str = "ops/org-migration.toml";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RepositoryManifest {
    #[serde(default)]
    pub repository: Vec<RepositoryRecord>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RepositoryRecord {
    pub id: String,
    pub github_slug: String,
    pub name: String,
    pub summary: String,
    pub class: RepositoryClass,
    pub status: RepositoryStatus,
    pub license: String,
    pub homepage: String,
    pub public: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RepositoryClass {
    Foundation,
    Platform,
    Product,
    Tool,
    MaintainedFork,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RepositoryStatus {
    Active,
    Prototype,
    Reference,
    Research,
    Archived,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelationManifest {
    #[serde(default)]
    pub relation: Vec<RelationRecord>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelationRecord {
    pub id: String,
    pub source: String,
    pub target: String,
    pub kind: RelationKind,
    pub provenance: RelationProvenance,
    pub evidence: String,
    pub verified_on: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationKind {
    DependsOn,
    Contains,
    ReferenceAppFor,
    HostFor,
    UsesUiFrom,
    RendersWith,
    ForkOf,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RelationProvenance {
    Derived,
    Curated,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MigrationManifest {
    pub inventory_receipt: String,
    #[serde(default)]
    pub migration: Vec<MigrationRecord>,
    #[serde(default)]
    pub unresolved_product: Vec<UnresolvedProduct>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MigrationRecord {
    pub id: String,
    pub current_slug: String,
    #[serde(default)]
    pub target_slug: Option<String>,
    pub classification: MigrationClass,
    pub batch: MigrationBatch,
    pub disposition: MigrationDisposition,
    pub visibility: Visibility,
    pub default_branch: String,
    pub head: String,
    pub license_status: String,
    pub provenance_status: String,
    pub sensitive_information_status: String,
    pub pages_status: String,
    pub packages_status: String,
    pub actions_workflows: u32,
    #[serde(default)]
    pub old_owner_files: Option<u32>,
    #[serde(default)]
    pub old_owner_manifests: Option<u32>,
    #[serde(default)]
    pub local_locator: Option<String>,
    #[serde(default)]
    pub source_aliases: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MigrationClass {
    Foundation,
    Platform,
    Product,
    Tool,
    MaintainedFork,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MigrationBatch {
    Infrastructure,
    Foundation,
    Platform,
    Product,
    ForkReview,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MigrationDisposition {
    AlreadyInOrg,
    Candidate,
    Hold,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UnresolvedProduct {
    pub name: String,
    pub state: String,
    pub evidence: String,
}

#[derive(Clone, Debug)]
pub struct Authority {
    pub repositories: RepositoryManifest,
    pub relations: RelationManifest,
    pub migration: MigrationManifest,
}

#[derive(Clone, Debug, Serialize)]
pub struct InventoryTarget {
    pub id: String,
    pub current_slug: String,
    pub classification: MigrationClass,
    pub batch: MigrationBatch,
    pub disposition: MigrationDisposition,
    pub expected_default_branch: String,
    pub expected_head: String,
    pub expected_pages_status: String,
    pub expected_packages_status: String,
    pub expected_actions_workflows: u32,
    pub expected_old_owner_files: Option<u32>,
    pub expected_old_owner_manifests: Option<u32>,
    pub local_locator: Option<String>,
    pub source_aliases: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct InventoryBasis {
    pub inventory_receipt: String,
    pub targets: Vec<InventoryTarget>,
    pub unresolved_products: Vec<UnresolvedProduct>,
}

#[derive(Debug)]
pub struct AuthorityError {
    context: String,
}

impl AuthorityError {
    fn new(context: impl Into<String>) -> Self {
        Self {
            context: context.into(),
        }
    }
}

impl fmt::Display for AuthorityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.context)
    }
}

impl std::error::Error for AuthorityError {}

impl Authority {
    pub fn load(root: impl AsRef<Path>) -> Result<Self, AuthorityError> {
        let root = root.as_ref();
        Ok(Self {
            repositories: read_toml(root.join(REPOSITORIES_PATH))?,
            relations: read_toml(root.join(RELATIONS_PATH))?,
            migration: read_toml(root.join(MIGRATION_PATH))?,
        })
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        let mut repository_ids = BTreeSet::new();
        let mut repository_slugs = BTreeSet::new();

        if self.repositories.repository.is_empty() {
            errors.push("content/repositories.toml has no repository records".to_owned());
        }

        for repository in &self.repositories.repository {
            check_nonempty("repository id", &repository.id, &mut errors);
            check_nonempty(
                &format!("repository {} name", repository.id),
                &repository.name,
                &mut errors,
            );
            check_nonempty(
                &format!("repository {} summary", repository.id),
                &repository.summary,
                &mut errors,
            );
            check_nonempty(
                &format!("repository {} license", repository.id),
                &repository.license,
                &mut errors,
            );
            check_https_or_github(
                &format!("repository {} homepage", repository.id),
                &repository.homepage,
                &mut errors,
            );
            check_slug(
                &format!("repository {} github_slug", repository.id),
                &repository.github_slug,
                &mut errors,
            );
            if !repository.public {
                errors.push(format!(
                    "repository {} is not public; private repositories do not belong in site authority",
                    repository.id
                ));
            }
            if !repository_ids.insert(repository.id.clone()) {
                errors.push(format!("duplicate repository id {}", repository.id));
            }
            if !repository_slugs.insert(repository.github_slug.clone()) {
                errors.push(format!(
                    "duplicate repository github_slug {}",
                    repository.github_slug
                ));
            }
        }

        let mut relation_ids = BTreeSet::new();
        for relation in &self.relations.relation {
            if !relation_ids.insert(relation.id.clone()) {
                errors.push(format!("duplicate relation id {}", relation.id));
            }
            if !repository_ids.contains(&relation.source) {
                errors.push(format!(
                    "relation {} has unknown source {}",
                    relation.id, relation.source
                ));
            }
            if !repository_ids.contains(&relation.target) {
                errors.push(format!(
                    "relation {} has unknown target {}",
                    relation.id, relation.target
                ));
            }
            if relation.source == relation.target {
                errors.push(format!("relation {} is a self-edge", relation.id));
            }
            check_nonempty(
                &format!("relation {} evidence", relation.id),
                &relation.evidence,
                &mut errors,
            );
            check_date(
                &format!("relation {} verified_on", relation.id),
                &relation.verified_on,
                &mut errors,
            );
        }

        let mut migration_ids = BTreeSet::new();
        let mut current_slugs = BTreeSet::new();
        let mut target_slugs = BTreeSet::new();
        let mut aliases = BTreeSet::new();
        let migrations_by_id: BTreeMap<_, _> = self
            .migration
            .migration
            .iter()
            .map(|migration| (migration.id.as_str(), migration))
            .collect();

        for migration in &self.migration.migration {
            if !migration_ids.insert(migration.id.clone()) {
                errors.push(format!("duplicate migration id {}", migration.id));
            }
            check_slug(
                &format!("migration {} current_slug", migration.id),
                &migration.current_slug,
                &mut errors,
            );
            if !current_slugs.insert(migration.current_slug.clone()) {
                errors.push(format!(
                    "duplicate migration current_slug {}",
                    migration.current_slug
                ));
            }
            if let Some(target_slug) = &migration.target_slug {
                check_slug(
                    &format!("migration {} target_slug", migration.id),
                    target_slug,
                    &mut errors,
                );
                if !target_slugs.insert(target_slug.clone()) {
                    errors.push(format!("duplicate migration target_slug {target_slug}"));
                }
            }
            for alias in &migration.source_aliases {
                check_slug(
                    &format!("migration {} source alias", migration.id),
                    alias,
                    &mut errors,
                );
                if alias == &migration.current_slug {
                    errors.push(format!(
                        "migration {} repeats its current slug as an alias",
                        migration.id
                    ));
                }
                if !aliases.insert(alias.clone()) {
                    errors.push(format!("duplicate migration source alias {alias}"));
                }
            }
            check_branch(&migration.id, &migration.default_branch, &mut errors);
            check_head(&migration.id, &migration.head, &mut errors);
            check_nonempty(
                &format!("migration {} license_status", migration.id),
                &migration.license_status,
                &mut errors,
            );
            check_nonempty(
                &format!("migration {} provenance_status", migration.id),
                &migration.provenance_status,
                &mut errors,
            );
            check_nonempty(
                &format!("migration {} sensitive_information_status", migration.id),
                &migration.sensitive_information_status,
                &mut errors,
            );
            check_nonempty(
                &format!("migration {} pages_status", migration.id),
                &migration.pages_status,
                &mut errors,
            );
            check_nonempty(
                &format!("migration {} packages_status", migration.id),
                &migration.packages_status,
                &mut errors,
            );
            if let Some(locator) = &migration.local_locator {
                check_safe_locator(&migration.id, locator, &mut errors);
            }

            match migration.disposition {
                MigrationDisposition::AlreadyInOrg => {
                    if !migration.current_slug.starts_with("merely-made/") {
                        errors.push(format!(
                            "migration {} says already-in-org but current slug is {}",
                            migration.id, migration.current_slug
                        ));
                    }
                    if migration.target_slug.as_deref() != Some(&migration.current_slug) {
                        errors.push(format!(
                            "migration {} already-in-org target must equal current slug",
                            migration.id
                        ));
                    }
                }
                MigrationDisposition::Candidate => {
                    let Some(target) = migration.target_slug.as_deref() else {
                        errors.push(format!(
                            "migration {} candidate has no target slug",
                            migration.id
                        ));
                        continue;
                    };
                    if !target.starts_with("merely-made/") {
                        errors.push(format!(
                            "migration {} candidate target is outside merely-made",
                            migration.id
                        ));
                    }
                }
                MigrationDisposition::Hold => {
                    if migration.target_slug.is_some() {
                        errors.push(format!(
                            "migration {} is on hold but already claims a target slug",
                            migration.id
                        ));
                    }
                }
            }
        }

        for repository in &self.repositories.repository {
            let Some(migration) = migrations_by_id.get(repository.id.as_str()) else {
                errors.push(format!(
                    "site repository {} has no migration ledger entry",
                    repository.id
                ));
                continue;
            };
            if migration.current_slug != repository.github_slug {
                errors.push(format!(
                    "repository {} slug {} disagrees with migration current slug {}",
                    repository.id, repository.github_slug, migration.current_slug
                ));
            }
        }

        check_receipt_path(&self.migration.inventory_receipt, &mut errors);

        let mut unresolved_names = BTreeSet::new();
        for unresolved in &self.migration.unresolved_product {
            check_nonempty("unresolved product name", &unresolved.name, &mut errors);
            check_nonempty(
                &format!("unresolved product {} state", unresolved.name),
                &unresolved.state,
                &mut errors,
            );
            check_nonempty(
                &format!("unresolved product {} evidence", unresolved.name),
                &unresolved.evidence,
                &mut errors,
            );
            if !unresolved_names.insert(unresolved.name.to_ascii_lowercase()) {
                errors.push(format!("duplicate unresolved product {}", unresolved.name));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            errors.sort();
            Err(errors)
        }
    }

    pub fn inventory_targets(&self) -> Vec<InventoryTarget> {
        self.migration
            .migration
            .iter()
            .map(|migration| InventoryTarget {
                id: migration.id.clone(),
                current_slug: migration.current_slug.clone(),
                classification: migration.classification,
                batch: migration.batch,
                disposition: migration.disposition,
                expected_default_branch: migration.default_branch.clone(),
                expected_head: migration.head.clone(),
                expected_pages_status: migration.pages_status.clone(),
                expected_packages_status: migration.packages_status.clone(),
                expected_actions_workflows: migration.actions_workflows,
                expected_old_owner_files: migration.old_owner_files,
                expected_old_owner_manifests: migration.old_owner_manifests,
                local_locator: migration.local_locator.clone(),
                source_aliases: migration.source_aliases.clone(),
            })
            .collect()
    }

    pub fn inventory_basis(&self) -> InventoryBasis {
        InventoryBasis {
            inventory_receipt: self.migration.inventory_receipt.clone(),
            targets: self.inventory_targets(),
            unresolved_products: self.migration.unresolved_product.clone(),
        }
    }
}

fn read_toml<T>(path: PathBuf) -> Result<T, AuthorityError>
where
    T: for<'de> Deserialize<'de>,
{
    let source = fs::read_to_string(&path)
        .map_err(|error| AuthorityError::new(format!("read {}: {error}", path.display())))?;
    toml::from_str(&source)
        .map_err(|error| AuthorityError::new(format!("parse {}: {error}", path.display())))
}

fn check_nonempty(label: &str, value: &str, errors: &mut Vec<String>) {
    if value.trim().is_empty() {
        errors.push(format!("{label} is empty"));
    }
}

fn check_slug(label: &str, value: &str, errors: &mut Vec<String>) {
    let mut parts = value.split('/');
    let owner = parts.next().unwrap_or_default();
    let repository = parts.next().unwrap_or_default();
    if owner.is_empty()
        || repository.is_empty()
        || parts.next().is_some()
        || !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.' | '/'))
    {
        errors.push(format!("{label} is not an owner/repository slug: {value}"));
    }
}

fn check_https_or_github(label: &str, value: &str, errors: &mut Vec<String>) {
    if !value.starts_with("https://") {
        errors.push(format!("{label} must use https: {value}"));
    }
}

fn check_date(label: &str, value: &str, errors: &mut Vec<String>) {
    let bytes = value.as_bytes();
    let valid = bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| matches!(index, 4 | 7) || byte.is_ascii_digit());
    if !valid {
        errors.push(format!("{label} is not YYYY-MM-DD: {value}"));
    }
}

fn check_branch(id: &str, branch: &str, errors: &mut Vec<String>) {
    if branch.is_empty()
        || branch.starts_with('/')
        || branch.ends_with('/')
        || branch.contains("..")
        || branch.contains(char::is_whitespace)
    {
        errors.push(format!(
            "migration {id} has invalid default branch {branch}"
        ));
    }
}

fn check_head(id: &str, head: &str, errors: &mut Vec<String>) {
    if head.len() != 40 || !head.chars().all(|ch| ch.is_ascii_hexdigit()) {
        errors.push(format!("migration {id} head is not a full git object id"));
    }
}

fn check_safe_locator(id: &str, locator: &str, errors: &mut Vec<String>) {
    let safe = !locator.is_empty()
        && !locator.starts_with('/')
        && !locator.starts_with('\\')
        && !locator.contains(':')
        && !locator.contains('\\')
        && !locator.split('/').any(|part| part == "..");
    if !safe {
        errors.push(format!(
            "migration {id} local_locator must be workspace-relative and slash-separated"
        ));
    }
}

fn check_receipt_path(path: &str, errors: &mut Vec<String>) {
    if !path.starts_with("docs/receipts/")
        || path.contains('\\')
        || path.contains(':')
        || path.split('/').any(|part| part == "..")
    {
        errors.push(format!(
            "inventory_receipt must be a safe path below docs/receipts: {path}"
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn workspace_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
    }

    #[test]
    fn committed_authority_is_valid() {
        let authority = Authority::load(workspace_root()).expect("load authority files");
        if let Err(errors) = authority.validate() {
            panic!("authority validation failed:\n{}", errors.join("\n"));
        }
    }

    #[test]
    fn inventory_targets_never_contain_absolute_paths() {
        let authority = Authority::load(workspace_root()).expect("load authority files");
        for target in authority.inventory_targets() {
            if let Some(locator) = target.local_locator {
                assert!(!locator.contains(':'));
                assert!(!locator.contains('\\'));
                assert!(!locator.starts_with('/'));
            }
        }
    }
}
