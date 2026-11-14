//! Site header with brand and main navigation.

use juoksevajohtaja_content::model::EPISODE_PATH_PREFIX;
use juoksevajohtaja_content::seo::{FEED_PATH, GAME_PATH};
use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::components::icon::{Icon, IconName};
use crate::data::PODCAST;

/// `aria-current="page"` value for a nav link. Episode pages belong to the episodes ("/") section.
pub fn current_marker(pathname: &str, target: &str) -> Option<&'static str> {
    let path = match pathname.trim_end_matches('/') {
        "" => "/",
        trimmed => trimmed,
    };
    let active = if target == "/" {
        path == "/" || path.starts_with(EPISODE_PATH_PREFIX)
    } else {
        path == target
    };
    active.then_some("page")
}

#[component]
pub fn SiteHeader() -> impl IntoView {
    let pathname = use_location().pathname;
    let marker = move |target: &'static str| move || current_marker(&pathname.get(), target);
    view! {
        <header class="site-header">
            <div class="site-header__inner">
                <a class="site-header__brand" href="/">
                    <Icon name=IconName::Headphones/>
                    <span>{PODCAST.title}</span>
                </a>
                <nav class="site-header__nav" aria-label="Päävalikko">
                    <a class="site-header__link" href="/" aria-current=marker("/")>"Jaksot"</a>
                    <a class="site-header__link" href=GAME_PATH aria-current=marker(GAME_PATH)>"Peli"</a>
                    <a class="site-header__link" href=FEED_PATH rel="external" title="RSS-syöte">
                        <Icon name=IconName::Rss/>
                        <span class="visually-hidden">"RSS-syöte"</span>
                    </a>
                </nav>
            </div>
        </header>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::render;

    #[test]
    fn marks_current_section() {
        assert_eq!(current_marker("/", "/"), Some("page"));
        assert_eq!(current_marker("", "/"), Some("page"));
        assert_eq!(current_marker("/jaksot/x", "/"), Some("page"));
        assert_eq!(current_marker("/peli/", "/peli"), Some("page"));
        assert_eq!(current_marker("/peli", "/"), None);
        assert_eq!(current_marker("/", "/peli"), None);
    }

    #[test]
    fn renders_navigation() {
        let html = render("/peli", || view! { <SiteHeader/> });
        assert!(html.contains("Juokseva Johtaja"));
        assert!(html.contains("href=\"/peli\" aria-current=\"page\""));
        assert!(html.contains("href=\"/feed.xml\" rel=\"external\""));
        assert_eq!(html.matches("aria-current").count(), 1);
    }
}
