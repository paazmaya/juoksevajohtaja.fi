//! Application root: router, layout and the site-wide player.

use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::use_location;
use leptos_router::path;

use crate::components::{AudioPlayer, SiteFooter, SiteHeader};
use crate::pages::{EpisodeRoute, GamePage, HomePage, NotFoundPage};
use crate::player::PlayerState;
use crate::web;

/// Reports route changes to analytics (browser only).
#[component]
fn NavigationTracker() -> impl IntoView {
    web::track_navigation(use_location().pathname);
}

/// The whole site. Only the `<Routes>` part changes on navigation; header, footer and the audio player stay
/// mounted, which keeps the podcast playing everywhere, including on the game page.
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    if use_context::<PlayerState>().is_none() {
        provide_context(PlayerState::new());
    }

    view! {
        <Router>
            <NavigationTracker/>
            <a class="skip-link" href="#main">"Siirry sisältöön"</a>
            <SiteHeader/>
            <main id="main" class="site-main">
                <Routes fallback=|| view! { <NotFoundPage/> }>
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/jaksot/:slug") view=EpisodeRoute/>
                    <Route path=path!("/peli") view=GamePage/>
                </Routes>
            </main>
            <SiteFooter/>
            <AudioPlayer/>
        </Router>
    }
}
