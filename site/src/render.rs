//! Build-time static rendering (feature `ssr`): every route becomes a real HTML file with route-specific
//! `<head>` tags, so deep links work on a static host and crawlers see complete pages.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use juoksevajohtaja_content::feed;
use juoksevajohtaja_content::seo::{self, PageMeta, GAME_PATH};
use leptos::prelude::*;
use leptos_meta::ServerMetaContext;
use leptos_router::location::RequestUrl;

use crate::app::App;
use crate::components::{LoaderVariant, RunnerLoader};
use crate::data::{EPISODES, PODCAST};
use crate::pages::GAME_APP_PATH;

/// Placeholder in `site/index.html` replaced by the rendered page.
pub const APP_MARKER: &str = "<!--app-->";
/// Placeholder in `site/index.html` replaced by the running-host loader.
pub const LOADER_MARKER: &str = "<!--boot-loader-->";

/// One page to write.
#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    /// Path rendered by the router.
    pub route: String,
    /// Output file relative to the dist directory.
    pub file: PathBuf,
    pub meta: PageMeta,
}

/// All pre-rendered pages: home, game, every episode and the 404 fallback.
pub fn pages() -> Vec<Page> {
    let mut pages = vec![
        Page {
            route: "/".into(),
            file: "index.html".into(),
            meta: seo::home(&PODCAST),
        },
        Page {
            route: GAME_PATH.into(),
            file: format!("{}/index.html", GAME_PATH.trim_start_matches('/')).into(),
            meta: seo::game(&PODCAST),
        },
    ];
    pages.extend(EPISODES.iter().map(|episode| Page {
        route: episode.path(),
        file: format!("{}/index.html", episode.path().trim_start_matches('/')).into(),
        meta: seo::episode(&PODCAST, episode),
    }));
    pages.push(Page {
        route: "/404".into(),
        file: "404.html".into(),
        meta: seo::not_found(&PODCAST),
    });
    pages
}

/// Renders the application body for a route.
pub fn render_route(path: &str) -> String {
    let owner = Owner::new();
    owner.with(|| {
        provide_context(RequestUrl::new(path));
        let (meta, _) = ServerMetaContext::new();
        provide_context(meta);
        view! { <App/> }.to_html()
    })
}

/// The running-host loader shown while the WebAssembly bundle loads.
pub fn render_boot_loader() -> String {
    let owner = Owner::new();
    owner
        .with(|| view! { <RunnerLoader label="Ladataan…" variant=LoaderVariant::Strip/> }.to_html())
}

/// Fills the Trunk-generated shell with head tags, page body and loader.
pub fn fill_template(
    template: &str,
    head: &str,
    body: &str,
    loader: &str,
) -> Result<String, String> {
    let mut html = template.to_string();
    for marker in [APP_MARKER, LOADER_MARKER, "</head>"] {
        if !html.contains(marker) {
            return Err(format!("template is missing {marker}"));
        }
    }
    // The shell's development <title> is replaced by the page title.
    if let (Some(start), Some(end)) = (html.find("<title>"), html.find("</title>")) {
        html.replace_range(start..end + "</title>".len(), "");
    }
    html = html.replacen("</head>", &format!("{head}</head>"), 1);
    html = html.replacen(APP_MARKER, body, 1);
    Ok(html.replacen(LOADER_MARKER, loader, 1))
}

/// Writes every page plus `feed.xml`, `sitemap.xml` and `robots.txt` into `dist`.
/// Returns the written files.
pub fn build_site(dist: &Path) -> io::Result<Vec<PathBuf>> {
    let template = fs::read_to_string(dist.join("index.html"))?;
    let loader = render_boot_loader();
    let mut written = Vec::new();
    let mut write = |relative: PathBuf, contents: String| -> io::Result<()> {
        let path = dist.join(&relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, contents)?;
        written.push(relative);
        Ok(())
    };
    for page in pages() {
        let html = fill_template(
            &template,
            &page.meta.head_html(&PODCAST),
            &render_route(&page.route),
            &loader,
        )
        .map_err(|message| io::Error::new(io::ErrorKind::InvalidData, message))?;
        write(page.file, html)?;
    }
    write("feed.xml".into(), feed::rss(&PODCAST, EPISODES))?;
    write("sitemap.xml".into(), feed::sitemap(&PODCAST, EPISODES))?;
    write(
        "robots.txt".into(),
        feed::robots(&PODCAST, &[GAME_APP_PATH]),
    )?;
    Ok(written)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEMPLATE: &str =
        "<html><head><title>Dev</title><link rel=\"modulepreload\" href=\"/app.js\"></head><body><div id=\"app\"><!--app--></div><div id=\"boot-loader\"><!--boot-loader--></div></body></html>";

    #[test]
    fn lists_pages() {
        let pages = pages();
        assert_eq!(pages.len(), EPISODES.len() + 3);
        assert_eq!(pages[0].file, PathBuf::from("index.html"));
        assert_eq!(pages[1].file, PathBuf::from("peli/index.html"));
        assert_eq!(
            pages[2].file,
            PathBuf::from(format!("jaksot/{}/index.html", EPISODES[0].slug))
        );
        assert_eq!(pages.last().unwrap().file, PathBuf::from("404.html"));
    }

    #[test]
    fn renders_routes() {
        let home = render_route("/");
        assert!(home.contains("home-hero"));
        assert!(home.contains("site-header"));
        assert!(home.contains("<audio"));
        let episode = render_route(&EPISODES[0].path());
        assert!(episode.contains(EPISODES[0].title));
        let game = render_route("/peli");
        assert!(game.contains("game-page"));
        let missing = render_route("/ei-ole");
        assert!(missing.contains("Sivua ei löytynyt"));
    }

    #[test]
    fn renders_loader() {
        let loader = render_boot_loader();
        assert!(loader.contains("runner-loader--strip"));
        assert!(loader.contains("Ladataan…"));
    }

    #[test]
    fn fills_template() {
        let html =
            fill_template(TEMPLATE, "<title>Page</title>", "<p>body</p>", "<i>run</i>").unwrap();
        assert!(!html.contains("Dev"));
        assert!(html.contains("<title>Page</title></head>"));
        assert!(html.contains("<div id=\"app\"><p>body</p></div>"));
        assert!(html.contains("<div id=\"boot-loader\"><i>run</i></div>"));
        assert!(html.contains("/app.js"));

        let without_title = TEMPLATE.replace("<title>Dev</title>", "");
        assert!(fill_template(&without_title, "", "", "").is_ok());
        assert_eq!(
            fill_template("<head></head>", "", "", "").unwrap_err(),
            "template is missing <!--app-->"
        );
    }

    #[test]
    fn builds_site_into_directory() {
        let dist = std::env::temp_dir().join(format!("jj-site-{}", std::process::id()));
        fs::create_dir_all(&dist).unwrap();
        assert!(build_site(&dist).is_err());

        fs::write(dist.join("index.html"), TEMPLATE).unwrap();
        let written = build_site(&dist).unwrap();
        assert!(written.contains(&PathBuf::from("feed.xml")));
        assert!(written.contains(&PathBuf::from("robots.txt")));
        let episode_file = dist.join(format!("jaksot/{}/index.html", EPISODES[0].slug));
        let episode_html = fs::read_to_string(episode_file).unwrap();
        assert!(episode_html.contains("application/ld+json"));
        assert!(episode_html.contains(EPISODES[0].title));
        assert!(episode_html.contains("runner-loader"));
        let not_found = fs::read_to_string(dist.join("404.html")).unwrap();
        assert!(not_found.contains("noindex"));
        assert!(fs::read_to_string(dist.join("robots.txt"))
            .unwrap()
            .contains("Disallow: /peli-app/"));

        fs::write(dist.join("index.html"), "<html><head></head></html>").unwrap();
        assert_eq!(
            build_site(&dist).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::remove_dir_all(&dist).unwrap();
    }
}
