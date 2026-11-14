//! Site footer with social links and credits.

use leptos::prelude::*;

use crate::data::PODCAST;

/// External links shown in the footer: (label, url).
pub const LINKS: [(&str, &str); 2] = [
    (
        "Twitter @juoksevajohtaja",
        "https://twitter.com/juoksevajohtaja",
    ),
    (
        "Lähdekoodi GitHubissa",
        "https://github.com/paazmaya/juoksevajohtaja.fi",
    ),
];

#[component]
pub fn SiteFooter() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <div class="site-footer__inner">
                <p class="site-footer__title">{PODCAST.title}" · "{PODCAST.author}</p>
                <ul class="site-footer__links" role="list">
                    {LINKS
                        .iter()
                        .map(|(label, url)| view! {
                            <li><a class="site-footer__link" href=*url rel="external noopener">{*label}</a></li>
                        })
                        .collect_view()}
                </ul>
                <p class="site-footer__credits">
                    "Pelin hahmot: "<a class="site-footer__link" href="https://www.kenney.nl/" rel="external noopener">"Kenney"</a>
                </p>
            </div>
        </footer>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::render;

    #[test]
    fn renders_links() {
        let html = render("/", || view! { <SiteFooter/> });
        for (label, url) in LINKS {
            assert!(html.contains(label));
            assert!(html.contains(url));
        }
        assert!(html.contains(PODCAST.author));
    }
}
