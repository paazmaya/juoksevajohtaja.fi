//! Generates Rust source with `static` podcast data, so the site compiles the catalogue into the binary.

use crate::model::{Catalog, Episode, Guest, Highlight, Podcast};

const CRATE: &str = "::juoksevajohtaja_content";

fn string(value: &str) -> String {
    // `Debug` for `str` produces a valid, escaped Rust string literal.
    format!("{value:?}")
}

fn optional_string(value: Option<&str>) -> String {
    match value {
        Some(value) => format!("Some({})", string(value)),
        None => "None".to_string(),
    }
}

fn optional_u32(value: Option<u32>) -> String {
    match value {
        Some(value) => format!("Some({value})"),
        None => "None".to_string(),
    }
}

fn podcast(podcast: &Podcast) -> String {
    let categories: Vec<String> = podcast.categories.iter().map(|c| string(c)).collect();
    format!(
        "{CRATE}::Podcast {{ title: {}, description: {}, author: {}, email: {}, language: {}, base_url: {}, image: {}, categories: &[{}] }}",
        string(podcast.title),
        string(podcast.description),
        string(podcast.author),
        optional_string(podcast.email),
        string(podcast.language),
        string(podcast.base_url),
        string(podcast.image),
        categories.join(", "),
    )
}

fn guest(guest: &Guest) -> String {
    format!(
        "{CRATE}::Guest {{ name: {}, role: {}, image: {} }}",
        string(guest.name),
        string(guest.role),
        optional_string(guest.image)
    )
}

fn highlight(highlight: &Highlight) -> String {
    format!(
        "{CRATE}::Highlight {{ time_seconds: {}, text: {} }}",
        optional_u32(highlight.time_seconds),
        string(highlight.text)
    )
}

fn episode(episode: &Episode) -> String {
    let highlights: Vec<String> = episode.highlights.iter().map(highlight).collect();
    format!(
        "{CRATE}::Episode {{ season: {}, number: {}, slug: {}, title: {}, \
         date: {CRATE}::Date {{ year: {}, month: {}, day: {} }}, image: {}, image_alt: {}, audio_url: {}, \
         audio_bytes: {}, duration_seconds: {}, summary_html: {}, summary_text: {}, intro_html: {}, \
         guest: {}, highlights: &[{}] }}",
        episode.season,
        episode.number,
        string(episode.slug),
        string(episode.title),
        episode.date.year,
        episode.date.month,
        episode.date.day,
        string(episode.image),
        string(episode.image_alt),
        string(episode.audio_url),
        episode.audio_bytes,
        episode.duration_seconds,
        string(episode.summary_html),
        string(episode.summary_text),
        string(episode.intro_html),
        guest(&episode.guest),
        highlights.join(", "),
    )
}

/// Rust source defining `PODCAST` and `EPISODES` statics.
pub fn rust_source(catalog: &Catalog) -> String {
    let episodes: Vec<String> = catalog
        .episodes
        .iter()
        .map(|e| format!("    {},\n", episode(e)))
        .collect();
    format!(
        "// @generated from episodes.yaml by site/build.rs. Do not edit.\n\
         /// Podcast metadata.\n\
         pub static PODCAST: {CRATE}::Podcast = {};\n\
         /// All episodes, newest first.\n\
         pub static EPISODES: &[{CRATE}::Episode] = &[\n{}];\n",
        podcast(&catalog.podcast),
        episodes.concat()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::fixtures;

    #[test]
    fn generates_rust_source() {
        let catalog = Catalog {
            podcast: fixtures::podcast(),
            episodes: vec![fixtures::episode()],
        };
        let source = rust_source(&catalog);
        assert!(source.contains("pub static PODCAST: ::juoksevajohtaja_content::Podcast"));
        assert!(source.contains("email: Some(\"podcast@example.com\")"));
        assert!(source.contains("categories: &[\"Business\", \"Management\"]"));
        assert!(source.contains("title: \"Testi \\\"jakso\\\"\""));
        assert!(source
            .contains("date: ::juoksevajohtaja_content::Date { year: 2026, month: 1, day: 15 }"));
        assert!(source.contains("image: None"));
        assert!(source.contains("time_seconds: Some(754)"));
        assert!(source.contains("time_seconds: None"));
    }

    #[test]
    fn generates_empty_episode_list() {
        let mut podcast = fixtures::podcast();
        podcast.email = None;
        let source = rust_source(&Catalog {
            podcast,
            episodes: vec![],
        });
        assert!(source.contains("email: None"));
        assert!(
            source.contains("pub static EPISODES: &[::juoksevajohtaja_content::Episode] = &[\n];")
        );
    }
}
