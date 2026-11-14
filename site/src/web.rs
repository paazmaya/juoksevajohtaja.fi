//! Browser-only glue: `<audio>` backend, Media Session, localStorage, analytics and boot clean-up.
//!
//! Everything here touches real browser APIs and is therefore exercised by the Playwright tests instead of
//! native unit tests (this file is excluded from the llvm-cov report). Keep logic out of this file: it belongs
//! in [`crate::player`] or the components, where it is unit-tested.

use leptos::prelude::*;

use crate::player::PlayerState;

/// localStorage key for the last played episode and position.
pub const STORAGE_KEY: &str = "juoksevajohtaja:player";

#[cfg(target_arch = "wasm32")]
mod browser {
    use std::rc::Rc;

    use leptos::prelude::*;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use web_sys::HtmlAudioElement;

    use super::STORAGE_KEY;
    use crate::data::{Episode, PODCAST};
    use crate::player::{AudioBackend, PlayerState, SKIP_BACK_SECONDS, SKIP_FORWARD_SECONDS};

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = window, js_name = ym, catch)]
        fn ym(counter: u32, action: &str, url: &str) -> Result<(), JsValue>;
    }

    // The Media Session API is still "unstable" in web-sys, so it is bound through a tiny JS snippet.
    #[wasm_bindgen(inline_js = "
        export function set_media_metadata(title, artist, album, artwork) {
            if (!(\"mediaSession\" in navigator) || typeof MediaMetadata === \"undefined\") return;
            navigator.mediaSession.metadata = new MediaMetadata({ title, artist, album, artwork: [{ src: artwork }] });
        }
        export function set_media_action(action, handler) {
            if (!(\"mediaSession\" in navigator)) return;
            try { navigator.mediaSession.setActionHandler(action, () => handler()); } catch (_) {}
        }
    ")]
    extern "C" {
        fn set_media_metadata(title: &str, artist: &str, album: &str, artwork: &str);
        fn set_media_action(action: &str, handler: &Closure<dyn Fn()>);
    }

    /// Yandex Metrika counter id (also in site/index.html).
    const METRIKA_COUNTER: u32 = 74_732_011;

    struct WebAudio(HtmlAudioElement);

    impl AudioBackend for WebAudio {
        fn load(&self, src: &str) {
            self.0.set_src(src);
        }
        fn play(&self) {
            // Rejections (autoplay policy) surface as a `pause` state; nothing else to do.
            let _ = self.0.play();
        }
        fn pause(&self) {
            let _ = self.0.pause();
        }
        fn seek(&self, seconds: f64) {
            self.0.set_current_time(seconds);
        }
        fn episode_changed(&self, episode: &'static Episode) {
            let artwork = juoksevajohtaja_content::model::asset_path(episode.image);
            set_media_metadata(episode.title, episode.guest.name, PODCAST.title, &artwork);
        }
    }

    fn storage() -> Option<web_sys::Storage> {
        window().local_storage().ok().flatten()
    }

    fn set_action(action: &str, handler: impl Fn() + 'static) {
        let closure = Closure::<dyn Fn()>::new(handler);
        set_media_action(action, &closure);
        closure.forget();
    }

    pub fn bind_audio(player: PlayerState, audio: NodeRef<leptos::html::Audio>) {
        Effect::new(move |attached: Option<bool>| {
            if attached == Some(true) {
                return true;
            }
            let Some(element) = audio.get() else {
                return false;
            };
            let element: HtmlAudioElement = element.unchecked_into();
            player.attach(Rc::new(WebAudio(element)));
            if let Some(saved) = storage().and_then(|s| s.get_item(STORAGE_KEY).ok().flatten()) {
                player.restore(&saved);
            }
            set_action("play", move || player.toggle());
            set_action("pause", move || player.pause());
            set_action("seekbackward", move || player.skip(-SKIP_BACK_SECONDS));
            set_action("seekforward", move || player.skip(SKIP_FORWARD_SECONDS));
            true
        });
        let snapshot = Memo::new(move |_| player.snapshot());
        Effect::new(move |_| {
            if let (Some(value), Some(storage)) = (snapshot.get(), storage()) {
                let _ = storage.set_item(STORAGE_KEY, &value);
            }
        });
    }

    pub fn track_navigation(pathname: Memo<String>) {
        Effect::new(move |previous: Option<String>| {
            let path = pathname.get();
            if previous.is_some_and(|p| p != path) {
                let _ = ym(METRIKA_COUNTER, "hit", &path);
            }
            path
        });
    }

    pub fn media_times(event: &web_sys::Event) -> (f64, f64) {
        event
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlMediaElement>().ok())
            .map(|media| (media.current_time(), media.duration()))
            .unwrap_or((0.0, 0.0))
    }

    pub fn focus_frame(event: &web_sys::Event) {
        if let Some(window) = event
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlIFrameElement>().ok())
            .and_then(|frame| frame.content_window())
        {
            let _ = window.focus();
        }
    }

    pub fn prepare_document() -> web_sys::HtmlElement {
        let document = document();
        // Static head tags from the pre-renderer are replaced by leptos_meta.
        if let Ok(nodes) = document.query_selector_all(&format!(
            "[{}]",
            juoksevajohtaja_content::seo::PRERENDERED_ATTR
        )) {
            for index in 0..nodes.length() {
                if let Some(node) = nodes
                    .item(index)
                    .and_then(|n| n.dyn_into::<web_sys::Element>().ok())
                {
                    node.remove();
                }
            }
        }
        let root = document
            .get_element_by_id("app")
            .expect("#app element in index.html");
        root.set_inner_html("");
        root.unchecked_into()
    }

    pub fn finish_boot() {
        if let Some(loader) = document().get_element_by_id("boot-loader") {
            loader.remove();
        }
        if let Some(body) = document().body() {
            let _ = body.class_list().add_1("is-ready");
        }
    }
}

/// Connects the `<audio>` element to the player, restores the saved position and wires lock-screen controls.
pub fn bind_audio(player: PlayerState, audio: NodeRef<leptos::html::Audio>) {
    #[cfg(target_arch = "wasm32")]
    browser::bind_audio(player, audio);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (player, audio);
}

/// Reports client-side navigations to analytics.
pub fn track_navigation(pathname: Memo<String>) {
    #[cfg(target_arch = "wasm32")]
    browser::track_navigation(pathname);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = pathname;
}

/// What components read from DOM events. Implemented here for `web_sys::Event`; unit tests use a fake, so
/// event handlers are testable without a browser.
pub trait DomEvent {
    /// `(currentTime, duration)` of the media element that fired the event.
    fn media_times(&self) -> (f64, f64);
    /// Value of the input element that fired the event.
    fn input_value(&self) -> String;
    /// Moves keyboard focus into the iframe that fired the event (game controls).
    fn focus_frame(&self);
}

impl DomEvent for web_sys::Event {
    fn media_times(&self) -> (f64, f64) {
        #[cfg(target_arch = "wasm32")]
        return browser::media_times(self);
        #[cfg(not(target_arch = "wasm32"))]
        (0.0, 0.0)
    }

    fn input_value(&self) -> String {
        event_target_value(self)
    }

    fn focus_frame(&self) {
        #[cfg(target_arch = "wasm32")]
        browser::focus_frame(self);
    }
}

/// Removes pre-rendered head tags and content; returns the mount point.
#[cfg(target_arch = "wasm32")]
pub fn prepare_document() -> web_sys::HtmlElement {
    browser::prepare_document()
}

/// Hides the running-host boot loader after the app has mounted.
#[cfg(target_arch = "wasm32")]
pub fn finish_boot() {
    browser::finish_boot()
}
