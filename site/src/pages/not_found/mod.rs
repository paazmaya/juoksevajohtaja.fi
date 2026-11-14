//! Fallback for unknown routes.

use juoksevajohtaja_content::seo;
use leptos::prelude::*;

use crate::components::{ButtonLink, IconName, PageHead, RunnerFigure};
use crate::data::PODCAST;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <PageHead meta=seo::not_found(&PODCAST)/>
        <section class="not-found">
            <RunnerFigure/>
            <h1 class="not-found__title">"Sivua ei löytynyt"</h1>
            <p class="not-found__text">"Taisit juosta harhaan reitiltä. Palataan lähtöviivalle."</p>
            <ButtonLink href="/" icon=IconName::ArrowLeft>"Etusivulle"</ButtonLink>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::render;

    #[test]
    fn renders_message() {
        let html = render("/missing", || view! { <NotFoundPage/> });
        assert!(html.contains("Sivua ei löytynyt"));
        assert!(html.contains("href=\"/\""));
        assert!(html.contains("class=\"runner\""));
    }
}
