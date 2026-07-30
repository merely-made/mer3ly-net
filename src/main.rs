use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use mer3ly_site::pages::{home, radio};
use mer3ly_site::site::SITE_CSS;

const OG_IMAGE: &[u8] = include_bytes!("../assets/og.jpg");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = output_directory()?;
    build_site(&output)?;
    println!("wrote static site to {}", output.display());
    Ok(())
}

fn output_directory() -> Result<PathBuf, String> {
    let mut args = env::args().skip(1);
    let mut output = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("html");
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output" => {
                let value = args
                    .next()
                    .ok_or_else(|| "--output requires a directory".to_owned())?;
                output = PathBuf::from(value);
            }
            "-h" | "--help" => {
                println!("Usage: cargo run --bin site -- [--output DIRECTORY]");
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    Ok(output)
}

fn build_site(output: &Path) -> std::io::Result<()> {
    fs::create_dir_all(output)?;
    fs::write(output.join("index.html"), home::document())?;
    fs::write(output.join("radio.html"), radio::document())?;
    fs::write(output.join("site.css"), SITE_CSS)?;
    fs::write(output.join("og.jpg"), OG_IMAGE)?;
    fs::write(output.join("CNAME"), "mer3ly.net\n")?;
    Ok(())
}
