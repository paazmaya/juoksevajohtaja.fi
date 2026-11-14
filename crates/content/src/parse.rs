//! Build-time reader for `episodes.yaml` and the episode markdown files.
//!
//! Parsed strings are leaked into `&'static str` so the result uses the same [`crate::model`] types as the
//! generated code. This only ever runs inside short-lived build processes.

use std::collections::HashSet;
use std::fmt;
use std::path::{Path, PathBuf};

use pulldown_cmark::{html, Event, Options, Parser, TagEnd};
use serde::Deserialize;

use crate::date::Date;
use crate::model::{Catalog, Episode, Guest, Highlight, Podcast};
use crate::time::parse_timestamp;

/// Heading that starts the episode summary section.
pub const SUMMARY_HEADING: &str = "Tiivistelmä";
/// Heading that starts the guest introduction section.
pub const INTRO_HEADING: &str = "Vieras";

/// Any problem with the content files. Messages name the offending file or episode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentError {
    Yaml(String),
    Read { path: String, message: String },
    Markdown { path: String, message: String },
    Invalid(String),
}

impl fmt::Display for ContentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yaml(message) => write!(f, "episodes.yaml: {message}"),
            Self::Read { path, message } => write!(f, "{path}: {message}"),
            Self::Markdown { path, message } => write!(f, "{path}: {message}"),
            Self::Invalid(message) => write!(f, "episodes.yaml: {message}"),
        }
    }
}

impl std::error::Error for ContentError {}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawFile {
    podcast: RawPodcast,
    episodes: Vec<RawEpisode>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawPodcast {
    title: String,
    description: String,
    author: String,
    email: Option<String>,
    language: String,
    base_url: String,
    image: String,
    #[serde(default)]
    categories: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawEpisode {
    season: u32,
    number: u32,
    slug: String,
    title: String,
    date: String,
    image: String,
    image_alt: String,
    audio_url: String,
    audio_bytes: u64,
    duration_seconds: u32,
    content: String,
    guest: RawGuest,
    #[serde(default)]
    highlights: Vec<RawHighlight>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawGuest {
    name: String,
    role: String,
    image: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawHighlight {
    time: Option<String>,
    text: String,
}

fn leak(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

fn leak_opt(value: Option<String>) -> Option<&'static str> {
    value.map(leak)
}

/// The two markdown sections of an episode file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sections {
    pub summary: String,
    pub intro: String,
}

/// Splits episode markdown into the required `## Tiivistelmä` and `## Vieras` sections.
pub fn split_sections(markdown: &str) -> Result<Sections, String> {
    let mut summary: Option<String> = None;
    let mut intro: Option<String> = None;
    let mut current: Option<&'static str> = None;
    let mut buffer = String::new();
    let mut in_fence = false;

    let mut flush = |section: Option<&'static str>, text: &mut String| -> Result<(), String> {
        let body = std::mem::take(text).trim().to_string();
        match section {
            None if body.is_empty() => Ok(()),
            None => Err(format!(
                "text before the first '## {SUMMARY_HEADING}' heading"
            )),
            Some(name) => {
                if body.is_empty() {
                    return Err(format!("section '## {name}' is empty"));
                }
                let slot = if name == SUMMARY_HEADING {
                    &mut summary
                } else {
                    &mut intro
                };
                if slot.is_some() {
                    return Err(format!("section '## {name}' appears twice"));
                }
                *slot = Some(body);
                Ok(())
            }
        }
    };

    for line in markdown.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
        }
        if !in_fence && line.starts_with("## ") {
            let heading = line[3..].trim();
            let section = if heading.eq_ignore_ascii_case(SUMMARY_HEADING) {
                SUMMARY_HEADING
            } else if heading.eq_ignore_ascii_case(INTRO_HEADING) {
                INTRO_HEADING
            } else {
                return Err(format!(
                    "unknown section '## {heading}', expected '## {SUMMARY_HEADING}' and '## {INTRO_HEADING}'"
                ));
            };
            flush(current, &mut buffer)?;
            current = Some(section);
            continue;
        }
        buffer.push_str(line);
        buffer.push('\n');
    }
    flush(current, &mut buffer)?;

    match (summary, intro) {
        (Some(summary), Some(intro)) => Ok(Sections { summary, intro }),
        (None, _) => Err(format!("missing section '## {SUMMARY_HEADING}'")),
        (_, None) => Err(format!("missing section '## {INTRO_HEADING}'")),
    }
}

fn markdown_options() -> Options {
    Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_SMART_PUNCTUATION
}

/// Renders markdown to HTML.
pub fn markdown_to_html(markdown: &str) -> String {
    let mut out = String::new();
    html::push_html(&mut out, Parser::new_ext(markdown, markdown_options()));
    out.trim_end().to_string()
}

/// Extracts the readable text of markdown, whitespace-normalised.
pub fn markdown_to_text(markdown: &str) -> String {
    let mut out = String::new();
    for event in Parser::new_ext(markdown, markdown_options()) {
        match event {
            Event::Text(text) | Event::Code(text) => out.push_str(&text),
            Event::SoftBreak | Event::HardBreak | Event::End(TagEnd::Paragraph | TagEnd::Item) => {
                out.push(' ')
            }
            _ => {}
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn is_valid_slug(slug: &str) -> bool {
    !slug.is_empty()
        && !slug.starts_with('-')
        && !slug.ends_with('-')
        && slug
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

fn is_remote(path: &str) -> bool {
    path.starts_with("https://") || path.starts_with("http://")
}

/// Parses `episodes.yaml` content.
///
/// * `read_markdown` returns the markdown file for a `content:` path.
/// * `asset_exists` tells whether a site-relative image path exists in the public directory.
pub fn parse_catalog(
    yaml: &str,
    read_markdown: impl Fn(&str) -> Result<String, String>,
    asset_exists: impl Fn(&str) -> bool,
) -> Result<Catalog, ContentError> {
    let raw: RawFile =
        serde_norway::from_str(yaml).map_err(|e| ContentError::Yaml(e.to_string()))?;

    if !raw.podcast.base_url.starts_with("https://") {
        return Err(ContentError::Invalid(
            "podcast.base_url must start with https://".into(),
        ));
    }
    let check_asset = |owner: &str, path: &str| -> Result<(), ContentError> {
        if is_remote(path) || asset_exists(path) {
            Ok(())
        } else {
            Err(ContentError::Invalid(format!(
                "{owner}: image '{path}' not found in site/public"
            )))
        }
    };
    check_asset("podcast", &raw.podcast.image)?;

    let podcast = Podcast {
        title: leak(raw.podcast.title),
        description: leak(raw.podcast.description),
        author: leak(raw.podcast.author),
        email: leak_opt(raw.podcast.email),
        language: leak(raw.podcast.language),
        base_url: leak(raw.podcast.base_url),
        image: leak(raw.podcast.image),
        categories: Box::leak(
            raw.podcast
                .categories
                .into_iter()
                .map(leak)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        ),
    };

    let mut slugs = HashSet::new();
    let mut numbers = HashSet::new();
    let mut episodes = Vec::with_capacity(raw.episodes.len());

    for item in raw.episodes {
        let owner = format!("episode '{}'", item.slug);
        let invalid = |message: String| ContentError::Invalid(format!("{owner}: {message}"));

        if !is_valid_slug(&item.slug) {
            return Err(invalid(
                "slug may only contain a-z, 0-9 and inner '-'".into(),
            ));
        }
        if !slugs.insert(item.slug.clone()) {
            return Err(invalid("duplicate slug".into()));
        }
        if !numbers.insert((item.season, item.number)) {
            return Err(invalid(format!(
                "duplicate season {} number {}",
                item.season, item.number
            )));
        }
        let date = Date::parse(&item.date).map_err(|e| invalid(e.to_string()))?;
        if !(item.audio_url.starts_with("https://") || item.audio_url.starts_with('/')) {
            return Err(invalid(
                "audio_url must be an https:// URL or a site path".into(),
            ));
        }
        if item.duration_seconds == 0 {
            return Err(invalid("duration_seconds must be positive".into()));
        }
        check_asset(&owner, &item.image)?;
        if let Some(image) = &item.guest.image {
            check_asset(&owner, image)?;
        }

        let mut highlights = Vec::with_capacity(item.highlights.len());
        for raw_highlight in item.highlights {
            let time_seconds = match &raw_highlight.time {
                Some(time) => {
                    let seconds = parse_timestamp(time).map_err(|e| invalid(e.to_string()))?;
                    if seconds > item.duration_seconds {
                        return Err(invalid(format!(
                            "highlight time {time} is past the episode end"
                        )));
                    }
                    Some(seconds)
                }
                None => None,
            };
            highlights.push(Highlight {
                time_seconds,
                text: leak(raw_highlight.text),
            });
        }

        let markdown = read_markdown(&item.content).map_err(|message| ContentError::Read {
            path: item.content.clone(),
            message,
        })?;
        let sections = split_sections(&markdown).map_err(|message| ContentError::Markdown {
            path: item.content.clone(),
            message,
        })?;

        episodes.push(Episode {
            season: item.season,
            number: item.number,
            slug: leak(item.slug),
            title: leak(item.title),
            date,
            image: leak(item.image),
            image_alt: leak(item.image_alt),
            audio_url: leak(item.audio_url),
            audio_bytes: item.audio_bytes,
            duration_seconds: item.duration_seconds,
            summary_html: leak(markdown_to_html(&sections.summary)),
            summary_text: leak(markdown_to_text(&sections.summary)),
            intro_html: leak(markdown_to_html(&sections.intro)),
            guest: Guest {
                name: leak(item.guest.name),
                role: leak(item.guest.role),
                image: leak_opt(item.guest.image),
            },
            highlights: Box::leak(highlights.into_boxed_slice()),
        });
    }

    episodes.sort_by(|a, b| (b.date, b.season, b.number).cmp(&(a.date, a.season, a.number)));
    Ok(Catalog { podcast, episodes })
}

/// Result of [`load`]: the catalogue and every file it was built from (for `cargo:rerun-if-changed`).
#[derive(Debug)]
pub struct Loaded {
    pub catalog: Catalog,
    pub inputs: Vec<PathBuf>,
}

/// Reads `<root>/episodes.yaml`, markdown paths relative to `root`, and checks images under `public_dir`.
pub fn load(root: &Path, public_dir: &Path) -> Result<Loaded, ContentError> {
    let yaml_path = root.join("episodes.yaml");
    let yaml = std::fs::read_to_string(&yaml_path).map_err(|e| ContentError::Read {
        path: yaml_path.display().to_string(),
        message: e.to_string(),
    })?;
    let inputs = std::cell::RefCell::new(vec![yaml_path]);
    let catalog = parse_catalog(
        &yaml,
        |relative| {
            let path = root.join(relative);
            inputs.borrow_mut().push(path.clone());
            std::fs::read_to_string(&path).map_err(|e| e.to_string())
        },
        |relative| public_dir.join(relative.trim_start_matches('/')).is_file(),
    )?;
    Ok(Loaded {
        catalog,
        inputs: inputs.into_inner(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const MARKDOWN: &str =
        "## Tiivistelmä\n\nJakso **johtamisesta**.\n\n## Vieras\n\nVieras on *CTO*.\n";

    fn yaml(episodes: &str) -> String {
        format!(
            "podcast:\n  title: Juokseva Johtaja\n  description: Kuvaus\n  author: Jukka\n  language: fi\n  \
             base_url: https://juoksevajohtaja.fi\n  image: images/cover.png\n  categories: [Business]\nepisodes:\n{episodes}"
        )
    }

    fn episode_yaml(slug: &str, number: u32, date: &str) -> String {
        format!(
            "  - season: 1\n    number: {number}\n    slug: {slug}\n    title: Jakso {number}\n    date: {date}\n    \
             image: images/e.png\n    image_alt: Kuva\n    audio_url: https://cdn.example.com/{slug}.mp3\n    \
             audio_bytes: 100\n    duration_seconds: 1200\n    content: content/{slug}.md\n    \
             guest:\n      name: Vieras\n      role: CTO\n    highlights:\n      - time: \"1:30\"\n        text: Alku\n      \
             - text: Ilman aikaa\n"
        )
    }

    fn parse(yaml: &str) -> Result<Catalog, ContentError> {
        parse_catalog(yaml, |_| Ok(MARKDOWN.to_string()), |_| true)
    }

    #[test]
    fn parses_and_sorts_newest_first() {
        let source =
            yaml(&(episode_yaml("eka", 1, "2026-01-01") + &episode_yaml("toka", 2, "2026-02-01")));
        let catalog = parse(&source).unwrap();
        assert_eq!(catalog.podcast.title, "Juokseva Johtaja");
        assert_eq!(catalog.podcast.categories, &["Business"]);
        assert_eq!(catalog.podcast.email, None);
        let slugs: Vec<_> = catalog.episodes.iter().map(|e| e.slug).collect();
        assert_eq!(slugs, ["toka", "eka"]);
        let episode = &catalog.episodes[1];
        assert_eq!(
            episode.summary_html,
            "<p>Jakso <strong>johtamisesta</strong>.</p>"
        );
        assert_eq!(episode.summary_text, "Jakso johtamisesta.");
        assert_eq!(episode.intro_html, "<p>Vieras on <em>CTO</em>.</p>");
        assert_eq!(episode.highlights[0].time_seconds, Some(90));
        assert_eq!(episode.highlights[1].time_seconds, None);
        assert_eq!(episode.guest.image, None);
    }

    #[test]
    fn reports_validation_errors() {
        let cases = [
            (
                yaml(&episode_yaml("Bad_Slug", 1, "2026-01-01")),
                "slug may only contain",
            ),
            (
                yaml(&(episode_yaml("a", 1, "2026-01-01") + &episode_yaml("a", 2, "2026-01-01"))),
                "duplicate slug",
            ),
            (
                yaml(&(episode_yaml("a", 1, "2026-01-01") + &episode_yaml("b", 1, "2026-01-01"))),
                "duplicate season",
            ),
            (yaml(&episode_yaml("a", 1, "2026-13-01")), "invalid date"),
            (
                yaml(&episode_yaml("a", 1, "2026-01-01")).replace("https://cdn", "ftp://cdn"),
                "audio_url",
            ),
            (
                yaml(&episode_yaml("a", 1, "2026-01-01"))
                    .replace("duration_seconds: 1200", "duration_seconds: 0"),
                "positive",
            ),
            (
                yaml(&episode_yaml("a", 1, "2026-01-01")).replace("\"1:30\"", "\"x\""),
                "invalid timestamp",
            ),
            (
                yaml(&episode_yaml("a", 1, "2026-01-01")).replace("\"1:30\"", "\"30:00\""),
                "past the episode end",
            ),
            (
                yaml(&episode_yaml("a", 1, "2026-01-01"))
                    .replace("https://juoksevajohtaja", "http://juoksevajohtaja"),
                "base_url",
            ),
            (
                yaml("").replace("episodes:\n", "episodes: []\nextra: 1\n"),
                "unknown field",
            ),
        ];
        for (source, expected) in cases {
            let error = parse(&source).unwrap_err().to_string();
            assert!(
                error.contains(expected),
                "expected '{expected}' in '{error}'"
            );
        }
    }

    #[test]
    fn accepts_site_relative_audio_and_guest_image() {
        let source = yaml(&episode_yaml("a", 1, "2026-01-01"))
            .replace("https://cdn.example.com/a.mp3", "/audio/a.mp3")
            .replace(
                "      role: CTO\n",
                "      role: CTO\n      image: https://x.fi/g.png\n",
            );
        let catalog = parse(&source).unwrap();
        assert_eq!(catalog.episodes[0].audio_url, "/audio/a.mp3");
        assert_eq!(catalog.episodes[0].guest.image, Some("https://x.fi/g.png"));
    }

    #[test]
    fn checks_that_images_exist() {
        let source = yaml(&episode_yaml("a", 1, "2026-01-01"));
        let error = parse_catalog(
            &source,
            |_| Ok(MARKDOWN.into()),
            |path| path != "images/e.png",
        )
        .unwrap_err();
        assert_eq!(
            error.to_string(),
            "episodes.yaml: episode 'a': image 'images/e.png' not found in site/public"
        );
        let error = parse_catalog(&source, |_| Ok(MARKDOWN.into()), |_| false).unwrap_err();
        assert!(error
            .to_string()
            .contains("podcast: image 'images/cover.png'"));
        let with_guest = source.replace(
            "      role: CTO\n",
            "      role: CTO\n      image: images/g.png\n",
        );
        let error = parse_catalog(
            &with_guest,
            |_| Ok(MARKDOWN.into()),
            |p| p != "images/g.png",
        )
        .unwrap_err();
        assert!(error.to_string().contains("images/g.png"));
    }

    #[test]
    fn reports_markdown_and_read_errors() {
        let source = yaml(&episode_yaml("a", 1, "2026-01-01"));
        let error = parse_catalog(&source, |_| Err("no such file".into()), |_| true).unwrap_err();
        assert_eq!(error.to_string(), "content/a.md: no such file");
        let error =
            parse_catalog(&source, |_| Ok("## Tiivistelmä\n\nX\n".into()), |_| true).unwrap_err();
        assert_eq!(
            error.to_string(),
            "content/a.md: missing section '## Vieras'"
        );
        let error = parse("::: not yaml").unwrap_err();
        assert!(matches!(error, ContentError::Yaml(_)));
        assert!(error.to_string().starts_with("episodes.yaml: "));
    }

    #[test]
    fn splits_sections() {
        let sections =
            split_sections("\n## Tiivistelmä\nA\n\n```\n## not a heading\n```\n## vieras\nB\n")
                .unwrap();
        assert_eq!(sections.summary, "A\n\n```\n## not a heading\n```");
        assert_eq!(sections.intro, "B");
    }

    #[test]
    fn rejects_bad_sections() {
        let cases = [
            ("Intro\n## Tiivistelmä\nA\n## Vieras\nB", "text before"),
            ("## Tiivistelmä\n\n## Vieras\nB", "is empty"),
            (
                "## Tiivistelmä\nA\n## Tiivistelmä\nA\n## Vieras\nB",
                "appears twice",
            ),
            ("## Tiivistelmä\nA\n## Muuta\nB", "unknown section"),
            ("## Vieras\nB", "missing section '## Tiivistelmä'"),
            ("## Tiivistelmä\nA", "missing section '## Vieras'"),
        ];
        for (markdown, expected) in cases {
            let error = split_sections(markdown).unwrap_err();
            assert!(
                error.contains(expected),
                "expected '{expected}' in '{error}'"
            );
        }
    }

    #[test]
    fn converts_markdown() {
        assert_eq!(
            markdown_to_html("a \"b\"\n\n- x\n- y"),
            "<p>a “b”</p>\n<ul>\n<li>x</li>\n<li>y</li>\n</ul>"
        );
        assert_eq!(
            markdown_to_text("Rivi\nkaksi `koodi`\n\n- x\n- y"),
            "Rivi kaksi koodi x y"
        );
    }

    #[test]
    fn validates_slugs() {
        assert!(is_valid_slug("jakso-1"));
        assert!(!is_valid_slug(""));
        assert!(!is_valid_slug("-a"));
        assert!(!is_valid_slug("a-"));
        assert!(!is_valid_slug("ä"));
    }

    #[test]
    fn loads_from_disk() {
        let dir = std::env::temp_dir().join(format!("jj-content-{}", std::process::id()));
        let public = dir.join("public");
        std::fs::create_dir_all(dir.join("content")).unwrap();
        std::fs::create_dir_all(public.join("images")).unwrap();
        std::fs::write(public.join("images/cover.png"), b"x").unwrap();
        std::fs::write(public.join("images/e.png"), b"x").unwrap();
        std::fs::write(dir.join("content/a.md"), MARKDOWN).unwrap();
        std::fs::write(
            dir.join("episodes.yaml"),
            yaml(&episode_yaml("a", 1, "2026-01-01")),
        )
        .unwrap();

        let loaded = load(&dir, &public).unwrap();
        assert_eq!(loaded.catalog.episodes.len(), 1);
        assert_eq!(
            loaded.inputs,
            vec![dir.join("episodes.yaml"), dir.join("content/a.md")]
        );

        std::fs::remove_file(dir.join("content/a.md")).unwrap();
        assert!(matches!(
            load(&dir, &public),
            Err(ContentError::Read { .. })
        ));
        let error = load(&dir.join("missing"), &public).unwrap_err();
        assert!(error.to_string().contains("episodes.yaml"));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
