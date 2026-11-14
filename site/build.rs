//! Compiles `episodes.yaml` and `content/*.md` into Rust statics (`OUT_DIR/episodes.rs`).

use std::path::PathBuf;

fn main() {
    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let root = manifest_dir
        .parent()
        .expect("site/ has a parent directory")
        .to_path_buf();
    let public_dir = manifest_dir.join("public");

    println!(
        "cargo:rerun-if-changed={}",
        root.join("episodes.yaml").display()
    );
    println!("cargo:rerun-if-changed={}", root.join("content").display());
    println!(
        "cargo:rerun-if-changed={}",
        public_dir.join("images").display()
    );

    let loaded = match juoksevajohtaja_content::parse::load(&root, &public_dir) {
        Ok(loaded) => loaded,
        Err(error) => {
            eprintln!("error: {error}");
            panic!("invalid podcast content: {error}");
        }
    };
    for input in &loaded.inputs {
        println!("cargo:rerun-if-changed={}", input.display());
    }

    let out = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR")).join("episodes.rs");
    std::fs::write(
        &out,
        juoksevajohtaja_content::codegen::rust_source(&loaded.catalog),
    )
    .expect("write generated episodes.rs");
}
