//! Emit a bounded, reduced public history payload from its checked-in sources.

use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use mer3ly_site::repositories::PublicSiteData;
use mer3ly_site::repository_history::{RepositoryGraph, public_history_projection};

const DEFAULT_POINTS: usize = 24;

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
    let root = arguments
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")));
    let points = arguments
        .next()
        .map(|value| {
            value
                .parse::<usize>()
                .map_err(|error| format!("invalid history point count: {error}"))
        })
        .transpose()?
        .unwrap_or(DEFAULT_POINTS);
    if arguments.next().is_some() {
        return Err("usage: repository-history [authority-root] [max-points]".to_owned());
    }

    let data = PublicSiteData::load(&root).map_err(|error| error.to_string())?;
    let current = RepositoryGraph::from_parts(
        &data.authority.repositories,
        &data.authority.relations,
        &data.metadata,
    )?;
    let projection = public_history_projection(&root, current, points)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&projection)
            .map_err(|error| format!("could not serialize Git authority history: {error}"))?
    );
    Ok(())
}
