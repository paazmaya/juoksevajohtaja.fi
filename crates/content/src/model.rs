//! Static podcast data model shared by build script, pre-renderer and the WebAssembly client.

use crate::date::Date;

/// Podcast-level metadata (`podcast:` in `episodes.yaml`).
#[derive(Debug, Clone, PartialEq)]
pub struct Podcast {
    pub title: &'static str,
    pub description: &'static str,
    pub author: &'static str,
    pub email: Option<&'static str>,
    pub language: &'static str,
    pub base_url: &'static str,
    /// Path relative to the site root, e.g. `images/podcast-cover.png`.
    pub image: &'static str,
    pub categories: &'static [&'static str],
}

/// The person being interviewed.
#[derive(Debug, Clone, PartialEq)]
pub struct Guest {
    pub name: &'static str,
    pub role: &'static str,
    pub image: Option<&'static str>,
}

/// A notable moment in an episode, optionally with a timestamp the player can seek to.
#[derive(Debug, Clone, PartialEq)]
pub struct Highlight {
    pub time_seconds: Option<u32>,
    pub text: &'static str,
}

/// One podcast episode with metadata from `episodes.yaml` and texts from its markdown file.
#[derive(Debug, Clone, PartialEq)]
pub struct Episode {
    pub season: u32,
    pub number: u32,
    pub slug: &'static str,
    pub title: &'static str,
    pub date: Date,
    pub image: &'static str,
    pub image_alt: &'static str,
    pub audio_url: &'static str,
    pub audio_bytes: u64,
    pub duration_seconds: u32,
    /// `## Tiivistelmä` rendered to HTML.
    pub summary_html: &'static str,
    /// `## Tiivistelmä` as plain text (meta description, RSS, cards).
    pub summary_text: &'static str,
    /// `## Vieras` rendered to HTML.
    pub intro_html: &'static str,
    pub guest: Guest,
    pub highlights: &'static [Highlight],
}

/// Path prefix of episode pages.
pub const EPISODE_PATH_PREFIX: &str = "/jaksot/";

impl Episode {
    /// Site-relative URL path of the episode page, e.g. `/jaksot/undisclosed`.
    pub fn path(&self) -> String {
        format!("{EPISODE_PATH_PREFIX}{}", self.slug)
    }

    /// `S1 · J3` style label.
    pub fn code(&self) -> String {
        format!("Kausi {} · Jakso {}", self.season, self.number)
    }
}

/// Everything parsed from `episodes.yaml`, episodes sorted newest first.
#[derive(Debug, Clone, PartialEq)]
pub struct Catalog {
    pub podcast: Podcast,
    pub episodes: Vec<Episode>,
}

/// Turns a site-relative asset path into an absolute path (`images/a.png` → `/images/a.png`).
/// Absolute URLs are returned unchanged.
pub fn asset_path(path: &str) -> String {
    if path.starts_with("http://") || path.starts_with("https://") || path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    }
}

/// Joins the site base URL and a path into an absolute URL.
pub fn absolute_url(base_url: &str, path: &str) -> String {
    if path.starts_with("http://") || path.starts_with("https://") {
        return path.to_string();
    }
    format!("{}{}", base_url.trim_end_matches('/'), asset_path(path))
}

/// Finds an episode by its slug.
pub fn find_by_slug<'a>(episodes: &'a [Episode], slug: &str) -> Option<&'a Episode> {
    episodes.iter().find(|episode| episode.slug == slug)
}

#[cfg(test)]
pub(crate) mod fixtures {
    use super::*;

    pub static HIGHLIGHTS: [Highlight; 2] = [
        Highlight {
            time_seconds: Some(754),
            text: "Juoksu & johtaminen",
        },
        Highlight {
            time_seconds: None,
            text: "Loppukiri",
        },
    ];

    pub static CATEGORIES: [&str; 2] = ["Business", "Management"];

    pub fn podcast() -> Podcast {
        Podcast {
            title: "Juokseva Johtaja",
            description: "Podcast <johtajuudesta>",
            author: "Jukka Paasonen",
            email: Some("podcast@example.com"),
            language: "fi",
            base_url: "https://juoksevajohtaja.fi/",
            image: "images/podcast-cover.png",
            categories: &CATEGORIES,
        }
    }

    pub fn episode() -> Episode {
        Episode {
            season: 1,
            number: 2,
            slug: "testi",
            title: "Testi \"jakso\"",
            date: Date {
                year: 2026,
                month: 1,
                day: 15,
            },
            image: "images/episodes/episode-2.svg",
            image_alt: "Kansikuva",
            audio_url: "https://cdn.example.com/e2.mp3",
            audio_bytes: 1234,
            duration_seconds: 2712,
            summary_html: "<p>Tiivistelmä</p>",
            summary_text: "Tiivistelmä",
            intro_html: "<p>Vieras</p>",
            guest: Guest {
                name: "Vieras Henkilö",
                role: "CTO",
                image: None,
            },
            highlights: &HIGHLIGHTS,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fixtures::*;
    use super::*;

    #[test]
    fn builds_paths() {
        let episode = episode();
        assert_eq!(episode.path(), "/jaksot/testi");
        assert_eq!(episode.code(), "Kausi 1 · Jakso 2");
    }

    #[test]
    fn builds_asset_paths() {
        assert_eq!(asset_path("images/a.png"), "/images/a.png");
        assert_eq!(asset_path("/images/a.png"), "/images/a.png");
        assert_eq!(asset_path("https://x.fi/a.png"), "https://x.fi/a.png");
        assert_eq!(asset_path("http://x.fi/a.png"), "http://x.fi/a.png");
    }

    #[test]
    fn builds_absolute_urls() {
        assert_eq!(
            absolute_url("https://a.fi/", "images/a.png"),
            "https://a.fi/images/a.png"
        );
        assert_eq!(absolute_url("https://a.fi", "/"), "https://a.fi/");
        assert_eq!(
            absolute_url("https://a.fi", "https://b.fi/x"),
            "https://b.fi/x"
        );
    }

    #[test]
    fn finds_by_slug() {
        let episodes = vec![episode()];
        assert_eq!(find_by_slug(&episodes, "testi").map(|e| e.number), Some(2));
        assert!(find_by_slug(&episodes, "missing").is_none());
    }
}
