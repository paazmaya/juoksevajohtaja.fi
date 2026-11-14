//! Game page. The game is a separate Trunk build loaded in an iframe, so its WebAssembly is only downloaded
//! when this page is opened, while the site header and podcast player keep running around it.

use juoksevajohtaja_content::seo;
use leptos::prelude::*;

use crate::components::{LoaderVariant, PageHead, RunnerLoader};
use crate::data::PODCAST;
use crate::web::DomEvent;

/// Public path of the separately built game (`game/Trunk.toml`).
pub const GAME_APP_PATH: &str = "/peli-app/";

/// Pre-rendered pages leave the iframe out: the browser creates it when the application mounts, so a
/// deep link to /peli does not start loading the game twice.
const EMBED_FRAME: bool = !cfg!(feature = "ssr");

/// iframe `load` handler: hide the loader and move keyboard focus into the game.
pub fn frame_loaded<E: DomEvent>(loaded: RwSignal<bool>) -> impl Fn(E) + 'static {
    move |event| {
        loaded.set(true);
        event.focus_frame();
    }
}

/// The game iframe.
#[component]
pub fn GameFrame(loaded: RwSignal<bool>) -> impl IntoView {
    view! {
        <iframe
            class="game-page__iframe"
            src=GAME_APP_PATH
            title="Juokseva Johtaja -juoksupeli"
            allow="autoplay; fullscreen"
            on:load=frame_loaded(loaded)
        ></iframe>
    }
}

#[component]
pub fn GamePage() -> impl IntoView {
    let loaded = RwSignal::new(false);
    view! {
        <PageHead meta=seo::game(&PODCAST)/>
        <section class="game-page">
            <div class="game-page__intro">
                <h1 class="game-page__title">"Juoksupeli"</h1>
                <p class="game-page__lead">
                    "Juokse kohti haasteita ja vastaa johtamiskysymyksiin. Podcast jatkaa soimista pelin aikana."
                </p>
            </div>
            <div class="game-page__frame">
                {EMBED_FRAME.then(|| view! { <GameFrame loaded=loaded/> })}
                <Show when=move || !loaded.get()>
                    <RunnerLoader label="Peli latautuu…" variant=LoaderVariant::Overlay/>
                </Show>
            </div>
            <p class="game-page__help">"Ohjaus: nuolinäppäimet ja välilyönti, mobiilissa kosketuspainikkeet."</p>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{render, with_owner, FakeEvent};

    #[test]
    fn prerenders_loader_without_iframe() {
        let html = render("/peli", || view! { <GamePage/> });
        assert!(html.contains("Juoksupeli"));
        assert!(html.contains("Peli latautuu…"));
        assert!(!html.contains("<iframe"));
    }

    #[test]
    fn renders_game_frame() {
        let html = render(
            "/peli",
            || view! { <GameFrame loaded=RwSignal::new(false)/> },
        );
        assert!(html.contains("src=\"/peli-app/\""));
        assert!(html.contains("allow=\"autoplay; fullscreen\""));
    }

    #[test]
    fn load_hides_loader_and_focuses_game() {
        with_owner(|| {
            let loaded = RwSignal::new(false);
            let event = FakeEvent::default();
            frame_loaded(loaded)(event.clone());
            assert!(loaded.get());
            assert!(event.focused.get());
        });
    }
}
