//! Helpers shared by unit tests.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, ServerMetaContext};
use leptos_router::components::Router;
use leptos_router::location::RequestUrl;

use crate::data::Episode;
use crate::player::{AudioBackend, PlayerState};
use crate::web::DomEvent;

/// Stand-in for a DOM event in handler tests.
#[derive(Clone, Default)]
pub struct FakeEvent {
    pub times: (f64, f64),
    pub value: String,
    pub focused: Rc<Cell<bool>>,
}

impl FakeEvent {
    pub fn media(current: f64, duration: f64) -> Self {
        Self {
            times: (current, duration),
            ..Self::default()
        }
    }

    pub fn input(value: &str) -> Self {
        Self {
            value: value.to_string(),
            ..Self::default()
        }
    }
}

impl DomEvent for FakeEvent {
    fn media_times(&self) -> (f64, f64) {
        self.times
    }
    fn input_value(&self) -> String {
        self.value.clone()
    }
    fn focus_frame(&self) {
        self.focused.set(true);
    }
}

/// Runs `f` inside a reactive owner, as components and signals require.
pub fn with_owner<T>(f: impl FnOnce() -> T) -> T {
    let owner = Owner::new();
    owner.with(f)
}

/// Records every call made by the player.
#[derive(Default)]
pub struct MockBackend {
    calls: RefCell<Vec<String>>,
}

impl MockBackend {
    pub fn calls(&self) -> Vec<String> {
        self.calls.borrow().clone()
    }
}

impl AudioBackend for MockBackend {
    fn load(&self, src: &str) {
        self.calls.borrow_mut().push(format!("load {src}"));
    }
    fn play(&self) {
        self.calls.borrow_mut().push("play".into());
    }
    fn pause(&self) {
        self.calls.borrow_mut().push("pause".into());
    }
    fn seek(&self, seconds: f64) {
        self.calls.borrow_mut().push(format!("seek {seconds}"));
    }
    fn episode_changed(&self, episode: &'static Episode) {
        self.calls
            .borrow_mut()
            .push(format!("meta {}", episode.slug));
    }
}

/// Provides a player with a mock backend in the current owner and returns both.
pub fn provide_mock_player() -> (PlayerState, Rc<MockBackend>) {
    let player = PlayerState::new();
    let backend = Rc::new(MockBackend::default());
    player.attach(backend.clone());
    provide_context(player);
    (player, backend)
}

/// Renders a view inside a `<Router>` at `path`, the way the pre-renderer does.
pub fn render<F, V>(path: &str, f: F) -> String
where
    F: FnOnce() -> V + Send + 'static,
    V: IntoView + 'static,
{
    render_with_head(path, f).0
}

/// Like [`render`], also returning the `<head>` content collected by `leptos_meta`.
pub fn render_with_head<F, V>(path: &str, f: F) -> (String, String)
where
    F: FnOnce() -> V + Send + 'static,
    V: IntoView + 'static,
{
    use futures::StreamExt;

    let _ = any_spawner::Executor::init_futures_executor();
    with_owner(|| {
        provide_context(RequestUrl::new(path));
        let (meta, output) = ServerMetaContext::new();
        provide_context(meta);
        provide_meta_context();
        let body = view! { <Router>{f()}</Router> }.to_html();
        let shell = format!("<html><head></head><body>{body}</body></html>");
        let document = futures::executor::block_on(async move {
            let stream = output
                .inject_meta_context(futures::stream::once(async move { shell }).boxed())
                .await;
            stream.collect::<Vec<_>>().await.concat()
        });
        let head = document
            .split("</head>")
            .next()
            .unwrap_or_default()
            .to_string();
        (body, head)
    })
}
