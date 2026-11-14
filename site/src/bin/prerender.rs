//! Pre-renders every route of the Trunk build into static HTML and writes feed.xml, sitemap.xml and robots.txt.
//!
//! Usage: `cargo run -p juoksevajohtaja-site --bin prerender --release -- [dist-dir]` (default `dist`).

use std::path::PathBuf;

fn main() {
    let dist = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("dist"));
    match juoksevajohtaja_site::render::build_site(&dist) {
        Ok(files) => {
            for file in files {
                println!("prerendered {}", dist.join(file).display());
            }
        }
        Err(error) => {
            eprintln!("prerender failed in {}: {error}", dist.display());
            std::process::exit(1);
        }
    }
}
