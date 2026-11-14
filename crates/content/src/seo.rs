//! Per-page SEO metadata shared by the pre-renderer (static `<head>` HTML) and the client (`leptos_meta`).

use crate::escape::{json_string, xml};
use crate::model::{absolute_url, asset_path, Episode, Podcast};
use crate::time::iso_duration;

/// Maximum length of a meta description before it is shortened.
pub const DESCRIPTION_MAX_CHARS: usize = 160;

/// Path of the game route.
pub const GAME_PATH: &str = "/peli";

/// Attribute on every pre-rendered head tag, so the client can drop them before `leptos_meta` takes over.
pub const PRERENDERED_ATTR: &str = "data-prerendered";

/// Path of the podcast RSS feed.
pub const FEED_PATH: &str = "/feed.xml";

/// Metadata of one page.
#[derive(Debug, Clone, PartialEq)]
pub struct PageMeta {
    /// Full document title.
    pub title: String,
    pub description: String,
    /// Site-relative canonical path.
    pub path: String,
    /// Site-relative or absolute image used for social previews.
    pub image: String,
    pub image_alt: String,
    /// Open Graph type: `website` or `article`.
    pub og_type: &'static str,
    /// Optional JSON-LD document.
    pub json_ld: Option<String>,
    /// `noindex` pages (404).
    pub noindex: bool,
}

/// Shortens text to at most `max` characters on a word boundary, adding an ellipsis when cut.
pub fn truncate_description(text: &str, max: usize) -> String {
    let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if normalized.chars().count() <= max {
        return normalized;
    }
    let cut: String = normalized.chars().take(max.saturating_sub(1)).collect();
    let trimmed = match cut.rfind(' ') {
        Some(index) if index > 0 => &cut[..index],
        _ => cut.as_str(),
    };
    format!("{}…", trimmed.trim_end_matches([',', '.', ';', ':']))
}

fn titled(page: &str, podcast: &Podcast) -> String {
    format!("{page} | {}", podcast.title)
}

/// Home page with `PodcastSeries` structured data.
pub fn home(podcast: &Podcast) -> PageMeta {
    let url = absolute_url(podcast.base_url, "/");
    let json_ld = format!(
        r#"{{"@context":"https://schema.org","@type":"PodcastSeries","name":{},"description":{},"url":{},"image":{},"inLanguage":{},"webFeed":{},"author":{{"@type":"Person","name":{}}}}}"#,
        json_string(podcast.title),
        json_string(podcast.description),
        json_string(&url),
        json_string(&absolute_url(podcast.base_url, podcast.image)),
        json_string(podcast.language),
        json_string(&absolute_url(podcast.base_url, FEED_PATH)),
        json_string(podcast.author),
    );
    PageMeta {
        title: format!("{} – johtajuuspodcast juoksulenkiltä", podcast.title),
        description: truncate_description(podcast.description, DESCRIPTION_MAX_CHARS),
        path: "/".to_string(),
        image: podcast.image.to_string(),
        image_alt: podcast.title.to_string(),
        og_type: "website",
        json_ld: Some(json_ld),
        noindex: false,
    }
}

/// Episode page with `PodcastEpisode` structured data.
pub fn episode(podcast: &Podcast, episode: &Episode) -> PageMeta {
    let url = absolute_url(podcast.base_url, &episode.path());
    let json_ld = format!(
        r#"{{"@context":"https://schema.org","@type":"PodcastEpisode","name":{},"description":{},"url":{},"image":{},"datePublished":{},"episodeNumber":{},"timeRequired":{},"partOfSeason":{{"@type":"PodcastSeason","seasonNumber":{}}},"partOfSeries":{{"@type":"PodcastSeries","name":{},"url":{}}},"associatedMedia":{{"@type":"AudioObject","contentUrl":{},"duration":{}}},"contributor":{{"@type":"Person","name":{},"jobTitle":{}}}}}"#,
        json_string(episode.title),
        json_string(episode.summary_text),
        json_string(&url),
        json_string(&absolute_url(podcast.base_url, episode.image)),
        json_string(&episode.date.iso()),
        episode.number,
        json_string(&iso_duration(episode.duration_seconds)),
        episode.season,
        json_string(podcast.title),
        json_string(&absolute_url(podcast.base_url, "/")),
        json_string(episode.audio_url),
        json_string(&iso_duration(episode.duration_seconds)),
        json_string(episode.guest.name),
        json_string(episode.guest.role),
    );
    PageMeta {
        title: titled(episode.title, podcast),
        description: truncate_description(episode.summary_text, DESCRIPTION_MAX_CHARS),
        path: episode.path(),
        image: episode.image.to_string(),
        image_alt: episode.image_alt.to_string(),
        og_type: "article",
        json_ld: Some(json_ld),
        noindex: false,
    }
}

/// Game page.
pub fn game(podcast: &Podcast) -> PageMeta {
    PageMeta {
        title: titled("Juoksupeli", podcast),
        description: "Pelaa Juokseva Johtaja -juoksupeliä: juokse kohti haasteita ja vastaa johtamiskysymyksiin. \
                      Podcast soi taustalla koko ajan."
            .to_string(),
        path: GAME_PATH.to_string(),
        image: podcast.image.to_string(),
        image_alt: podcast.title.to_string(),
        og_type: "website",
        json_ld: None,
        noindex: false,
    }
}

/// Not found page.
pub fn not_found(podcast: &Podcast) -> PageMeta {
    PageMeta {
        title: titled("Sivua ei löytynyt", podcast),
        description: "Etsimääsi sivua ei löytynyt.".to_string(),
        path: "/404".to_string(),
        image: podcast.image.to_string(),
        image_alt: podcast.title.to_string(),
        og_type: "website",
        json_ld: None,
        noindex: true,
    }
}

impl PageMeta {
    /// Absolute canonical URL.
    pub fn canonical_url(&self, podcast: &Podcast) -> String {
        absolute_url(podcast.base_url, &self.path)
    }

    /// Absolute social preview image URL.
    pub fn image_url(&self, podcast: &Podcast) -> String {
        absolute_url(podcast.base_url, &self.image)
    }

    /// Static `<head>` markup for pre-rendered pages.
    pub fn head_html(&self, podcast: &Podcast) -> String {
        let canonical = xml(&self.canonical_url(podcast));
        let image = xml(&self.image_url(podcast));
        let title = xml(&self.title);
        let description = xml(&self.description);
        let mut out = String::new();
        out.push_str(&format!("<title {PRERENDERED_ATTR}>{title}</title>\n"));
        out.push_str(&format!(
            "<meta {PRERENDERED_ATTR} name=\"description\" content=\"{description}\">\n"
        ));
        if self.noindex {
            out.push_str(&format!(
                "<meta {PRERENDERED_ATTR} name=\"robots\" content=\"noindex\">\n"
            ));
        } else {
            out.push_str(&format!(
                "<link {PRERENDERED_ATTR} rel=\"canonical\" href=\"{canonical}\">\n"
            ));
        }
        out.push_str(&format!(
            "<link {PRERENDERED_ATTR} rel=\"alternate\" type=\"application/rss+xml\" title=\"{}\" href=\"{}\">\n",
            xml(podcast.title),
            xml(&asset_path(FEED_PATH))
        ));
        out.push_str(&format!(
            "<meta {PRERENDERED_ATTR} property=\"og:type\" content=\"{}\">\n",
            self.og_type
        ));
        out.push_str(&format!(
            "<meta {PRERENDERED_ATTR} property=\"og:site_name\" content=\"{}\">\n",
            xml(podcast.title)
        ));
        out.push_str(&format!(
            "<meta {PRERENDERED_ATTR} property=\"og:locale\" content=\"{}\">\n",
            xml(&og_locale(podcast.language))
        ));
        out.push_str(&format!(
            "<meta {PRERENDERED_ATTR} property=\"og:title\" content=\"{title}\">\n"
        ));
        out.push_str(&format!(
            "<meta {PRERENDERED_ATTR} property=\"og:description\" content=\"{description}\">\n"
        ));
        out.push_str(&format!(
            "<meta {PRERENDERED_ATTR} property=\"og:url\" content=\"{canonical}\">\n"
        ));
        out.push_str(&format!(
            "<meta {PRERENDERED_ATTR} property=\"og:image\" content=\"{image}\">\n"
        ));
        out.push_str(&format!(
            "<meta {PRERENDERED_ATTR} property=\"og:image:alt\" content=\"{}\">\n",
            xml(&self.image_alt)
        ));
        out.push_str(&format!(
            "<meta {PRERENDERED_ATTR} name=\"twitter:card\" content=\"summary_large_image\">\n"
        ));
        out.push_str(&format!(
            "<meta {PRERENDERED_ATTR} name=\"twitter:title\" content=\"{title}\">\n"
        ));
        out.push_str(&format!(
            "<meta {PRERENDERED_ATTR} name=\"twitter:description\" content=\"{description}\">\n"
        ));
        out.push_str(&format!(
            "<meta {PRERENDERED_ATTR} name=\"twitter:image\" content=\"{image}\">\n"
        ));
        if let Some(json_ld) = &self.json_ld {
            out.push_str(&format!(
                "<script {PRERENDERED_ATTR} type=\"application/ld+json\">{json_ld}</script>\n"
            ));
        }
        out
    }
}

/// `fi` → `fi_FI`, `en-GB` → `en_GB`.
pub fn og_locale(language: &str) -> String {
    match language.split_once(['-', '_']) {
        Some((lang, region)) => format!("{}_{}", lang.to_lowercase(), region.to_uppercase()),
        None => format!("{}_{}", language.to_lowercase(), language.to_uppercase()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::fixtures as fx;

    #[test]
    fn truncates_descriptions() {
        assert_eq!(
            truncate_description("  lyhyt   teksti ", 160),
            "lyhyt teksti"
        );
        assert_eq!(
            truncate_description("yksi kaksi kolme neljä", 12),
            "yksi kaksi…"
        );
        assert_eq!(truncate_description("yksi, kaksi kolme", 8), "yksi…");
        assert_eq!(truncate_description("pitkäsanailmanvälejä", 6), "pitkä…");
    }

    #[test]
    fn builds_home_meta() {
        let podcast = fx::podcast();
        let meta = home(&podcast);
        assert_eq!(meta.path, "/");
        assert!(meta.title.starts_with("Juokseva Johtaja"));
        let json = meta.json_ld.clone().unwrap();
        assert!(json.contains("\"@type\":\"PodcastSeries\""));
        assert!(json.contains("https://juoksevajohtaja.fi/feed.xml"));
        assert!(json.contains("\\u003cjohtajuudesta>"));
        assert_eq!(meta.canonical_url(&podcast), "https://juoksevajohtaja.fi/");
    }

    #[test]
    fn builds_episode_meta() {
        let podcast = fx::podcast();
        let sample = fx::episode();
        let meta = episode(&podcast, &sample);
        assert_eq!(meta.title, "Testi \"jakso\" | Juokseva Johtaja");
        assert_eq!(meta.og_type, "article");
        assert_eq!(
            meta.image_url(&podcast),
            "https://juoksevajohtaja.fi/images/episodes/episode-2.svg"
        );
        let json = meta.json_ld.unwrap();
        assert!(json.contains("\"@type\":\"PodcastEpisode\""));
        assert!(json.contains("\"datePublished\":\"2026-01-15\""));
        assert!(json.contains("\"duration\":\"PT45M12S\""));
        assert!(json.contains("\"name\":\"Vieras Henkilö\""));
    }

    #[test]
    fn builds_game_and_not_found_meta() {
        let podcast = fx::podcast();
        assert_eq!(game(&podcast).path, "/peli");
        let missing = not_found(&podcast);
        assert!(missing.noindex);
        assert!(missing.json_ld.is_none());
    }

    #[test]
    fn renders_head_html() {
        let podcast = fx::podcast();
        let html = episode(&podcast, &fx::episode()).head_html(&podcast);
        assert!(html.contains(
            "<title data-prerendered>Testi &quot;jakso&quot; | Juokseva Johtaja</title>"
        ));
        assert!(html.contains("<link data-prerendered rel=\"canonical\" href=\"https://juoksevajohtaja.fi/jaksot/testi\">"));
        assert!(html.contains("og:locale\" content=\"fi_FI\""));
        assert!(html.contains("application/ld+json"));
        assert!(html.contains("href=\"/feed.xml\""));

        let html = not_found(&podcast).head_html(&podcast);
        assert!(html.contains("noindex"));
        assert!(!html.contains("canonical"));
        assert!(!html.contains("ld+json"));
    }

    #[test]
    fn maps_og_locale() {
        assert_eq!(og_locale("fi"), "fi_FI");
        assert_eq!(og_locale("en-gb"), "en_GB");
        assert_eq!(og_locale("sv_FI"), "sv_FI");
    }
}
