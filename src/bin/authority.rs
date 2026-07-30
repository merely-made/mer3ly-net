use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use mer3ly_site::repositories::{Authority, PublicMetadataCache};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut arguments = env::args().skip(1);
    let command = arguments.next().unwrap_or_else(|| "validate".to_owned());
    let root = arguments
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")));
    let metadata_path = arguments.next().map(PathBuf::from);
    if arguments.next().is_some() {
        return Err(
            "usage: authority [validate|summary|inventory-targets|public-repositories|validate-metadata] [root] [metadata-path]"
                .to_owned(),
        );
    }
    if command != "validate-metadata" && metadata_path.is_some() {
        return Err(format!("{command} does not accept a metadata path"));
    }

    let authority = Authority::load(&root).map_err(|error| error.to_string())?;
    authority
        .validate()
        .map_err(|errors| format!("authority validation failed:\n{}", errors.join("\n")))?;

    match command.as_str() {
        "validate" => {
            println!(
                "authority valid: {} repositories, {} relations, {} migration records, {} unresolved products",
                authority.repositories.repository.len(),
                authority.relations.relation.len(),
                authority.migration.migration.len(),
                authority.migration.unresolved_product.len()
            );
        }
        "summary" => {
            let candidates = authority
                .migration
                .migration
                .iter()
                .filter(|record| {
                    record.disposition == mer3ly_site::repositories::MigrationDisposition::Candidate
                })
                .count();
            let holds = authority
                .migration
                .migration
                .iter()
                .filter(|record| {
                    record.disposition == mer3ly_site::repositories::MigrationDisposition::Hold
                })
                .count();
            let personal_forks = authority
                .migration
                .migration
                .iter()
                .filter(|record| {
                    record.disposition
                        == mer3ly_site::repositories::MigrationDisposition::KeepPersonal
                })
                .count();
            println!(
                "{} public site repositories; {candidates} transfer candidates; {personal_forks} maintained forks kept personal; {holds} unresolved fork-review holds",
                authority.repositories.repository.len()
            );
        }
        "inventory-targets" => {
            println!(
                "{}",
                serde_json::to_string_pretty(&authority.inventory_basis())
                    .map_err(|error| format!("serialize inventory targets: {error}"))?
            );
        }
        "public-repositories" => {
            println!(
                "{}",
                serde_json::to_string_pretty(&authority.repositories)
                    .map_err(|error| format!("serialize public repositories: {error}"))?
            );
        }
        "validate-metadata" => {
            let path = metadata_path.unwrap_or_else(|| root.join("content/github-metadata.json"));
            let metadata = PublicMetadataCache::load(&path).map_err(|error| error.to_string())?;
            metadata.validate(&authority).map_err(|errors| {
                format!("public metadata validation failed:\n{}", errors.join("\n"))
            })?;
            println!(
                "public metadata valid: {} repositories refreshed {}",
                metadata.repository.len(),
                metadata.generated_at_utc
            );
        }
        _ => {
            return Err(format!(
                "unknown command {command}; expected validate, summary, inventory-targets, public-repositories, or validate-metadata"
            ));
        }
    }

    Ok(())
}
