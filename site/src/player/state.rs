//! Player model: reactive signals plus an [`AudioBackend`] that performs the actual playback.

use std::rc::Rc;

use leptos::prelude::*;

use crate::data::{episode_by_slug, Episode};

/// Seconds jumped by the "back" button.
pub const SKIP_BACK_SECONDS: f64 = 15.0;
/// Seconds jumped by the "forward" button.
pub const SKIP_FORWARD_SECONDS: f64 = 30.0;
/// Saved positions are rounded down to this many seconds, which also throttles storage writes.
const SNAPSHOT_GRANULARITY_SECONDS: f64 = 5.0;

/// Something that can play audio: the `<audio>` element in the browser, a mock in tests.
pub trait AudioBackend {
    fn load(&self, src: &str);
    fn play(&self);
    fn pause(&self);
    fn seek(&self, seconds: f64);
    /// Called when another episode is loaded (lock-screen metadata etc.).
    fn episode_changed(&self, _episode: &'static Episode) {}
}

/// Reactive player state. `Copy`, so it can be moved into any number of event handlers.
///
/// Each value is its own signal: the progress bar re-renders on `timeupdate` without touching the rest of the UI.
#[derive(Clone, Copy)]
pub struct PlayerState {
    current: RwSignal<Option<&'static Episode>>,
    playing: RwSignal<bool>,
    position: RwSignal<f64>,
    duration: RwSignal<f64>,
    backend: StoredValue<Option<Rc<dyn AudioBackend>>, LocalStorage>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::new()
    }
}

/// Parses a stored `slug@seconds` snapshot.
pub fn parse_snapshot(value: &str) -> Option<(&'static Episode, f64)> {
    let (slug, seconds) = value.rsplit_once('@')?;
    let seconds: f64 = seconds
        .parse()
        .ok()
        .filter(|s: &f64| s.is_finite() && *s >= 0.0)?;
    Some((episode_by_slug(slug)?, seconds))
}

impl PlayerState {
    pub fn new() -> Self {
        Self {
            current: RwSignal::new(None),
            playing: RwSignal::new(false),
            position: RwSignal::new(0.0),
            duration: RwSignal::new(0.0),
            backend: StoredValue::new_local(None),
        }
    }

    /// Connects the playback implementation.
    pub fn attach(&self, backend: Rc<dyn AudioBackend>) {
        self.backend.set_value(Some(backend));
    }

    fn with_backend(&self, action: impl FnOnce(&dyn AudioBackend)) {
        self.backend.with_value(|backend| {
            if let Some(backend) = backend {
                action(backend.as_ref());
            }
        });
    }

    pub fn current(&self) -> Option<&'static Episode> {
        self.current.get()
    }

    pub fn is_playing(&self) -> bool {
        self.playing.get()
    }

    pub fn position(&self) -> f64 {
        self.position.get()
    }

    pub fn duration(&self) -> f64 {
        self.duration.get()
    }

    /// Whether this episode is loaded and playing right now.
    pub fn is_playing_episode(&self, episode: &Episode) -> bool {
        self.playing.get()
            && self
                .current
                .with(|current| current.is_some_and(|c| c.slug == episode.slug))
    }

    fn is_current(&self, episode: &Episode) -> bool {
        self.current
            .with_untracked(|current| current.is_some_and(|c| c.slug == episode.slug))
    }

    fn load(&self, episode: &'static Episode, start_at: f64) {
        self.current.set(Some(episode));
        self.position.set(start_at);
        self.duration.set(f64::from(episode.duration_seconds));
        self.with_backend(|backend| {
            backend.load(episode.audio_url);
            backend.episode_changed(episode);
            if start_at > 0.0 {
                backend.seek(start_at);
            }
        });
    }

    /// Starts or resumes an episode.
    pub fn play_episode(&self, episode: &'static Episode) {
        if !self.is_current(episode) {
            self.load(episode, 0.0);
        }
        self.with_backend(|backend| backend.play());
    }

    /// Plays an episode from a given second (highlights).
    pub fn play_from(&self, episode: &'static Episode, seconds: f64) {
        if self.is_current(episode) {
            self.seek(seconds);
        } else {
            self.load(episode, seconds);
        }
        self.with_backend(|backend| backend.play());
    }

    /// Play/pause for the given episode: pauses if it is playing, otherwise plays it.
    pub fn toggle_episode(&self, episode: &'static Episode) {
        if self.is_current(episode) && self.playing.get_untracked() {
            self.pause();
        } else {
            self.play_episode(episode);
        }
    }

    /// Play/pause for whatever is loaded.
    pub fn toggle(&self) {
        if let Some(episode) = self.current.get_untracked() {
            self.toggle_episode(episode)
        }
    }

    pub fn pause(&self) {
        self.with_backend(|backend| backend.pause());
    }

    /// Jumps to an absolute position, clamped to the episode length.
    pub fn seek(&self, seconds: f64) {
        let max = self.duration.get_untracked().max(0.0);
        let target = if seconds.is_finite() {
            seconds.clamp(0.0, max)
        } else {
            0.0
        };
        self.position.set(target);
        self.with_backend(|backend| backend.seek(target));
    }

    /// Relative jump (negative = back).
    pub fn skip(&self, delta: f64) {
        self.seek(self.position.get_untracked() + delta);
    }

    // Events reported by the backend.

    pub fn on_play(&self) {
        self.playing.set(true);
    }

    pub fn on_pause(&self) {
        self.playing.set(false);
    }

    pub fn on_time_update(&self, seconds: f64) {
        if seconds.is_finite() {
            self.position.set(seconds);
        }
    }

    /// The media reports its real duration (may differ from the yaml estimate).
    pub fn on_duration(&self, seconds: f64) {
        if seconds.is_finite() && seconds > 0.0 {
            self.duration.set(seconds);
        }
    }

    pub fn on_ended(&self) {
        self.playing.set(false);
        self.position.set(0.0);
    }

    /// Progress 0.0–1.0.
    pub fn progress(&self) -> f64 {
        let duration = self.duration.get();
        if duration > 0.0 {
            (self.position.get() / duration).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    /// `slug@seconds` for persisting, coarse-grained so it changes only every few seconds.
    pub fn snapshot(&self) -> Option<String> {
        let episode = self.current.get()?;
        let position = self.position.get();
        let rounded =
            (position / SNAPSHOT_GRANULARITY_SECONDS).floor() * SNAPSHOT_GRANULARITY_SECONDS;
        Some(format!("{}@{}", episode.slug, rounded))
    }

    /// Restores a saved snapshot without starting playback.
    pub fn restore(&self, snapshot: &str) -> bool {
        match parse_snapshot(snapshot) {
            Some((episode, seconds)) => {
                let start = seconds.min(f64::from(episode.duration_seconds));
                self.load(episode, start);
                true
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::EPISODES;
    use crate::test_support::{with_owner, MockBackend};

    fn setup() -> (PlayerState, Rc<MockBackend>) {
        let player = PlayerState::new();
        let backend = Rc::new(MockBackend::default());
        player.attach(backend.clone());
        (player, backend)
    }

    #[test]
    fn plays_and_resumes_episode() {
        with_owner(|| {
            let (player, backend) = setup();
            let episode = &EPISODES[0];
            assert!(!player.is_playing_episode(episode));
            player.play_episode(episode);
            player.on_play();
            assert_eq!(player.current(), Some(episode));
            assert!(player.is_playing());
            assert!(player.is_playing_episode(episode));
            assert_eq!(player.duration(), f64::from(episode.duration_seconds));
            player.play_episode(episode);
            assert_eq!(
                backend.calls(),
                vec![
                    format!("load {}", episode.audio_url),
                    format!("meta {}", episode.slug),
                    "play".into(),
                    "play".into()
                ]
            );
        });
    }

    #[test]
    fn toggles_playback() {
        with_owner(|| {
            let (player, backend) = setup();
            player.toggle();
            assert!(backend.calls().is_empty());
            let episode = &EPISODES[0];
            player.toggle_episode(episode);
            player.on_play();
            player.toggle();
            player.on_pause();
            assert!(!player.is_playing());
            player.toggle();
            let calls = backend.calls();
            assert_eq!(&calls[2..], ["play", "pause", "play"]);
        });
    }

    #[test]
    fn seeks_and_skips_within_bounds() {
        with_owner(|| {
            let (player, backend) = setup();
            let episode = &EPISODES[0];
            player.play_episode(episode);
            player.seek(100.0);
            assert_eq!(player.position(), 100.0);
            player.skip(-SKIP_BACK_SECONDS);
            assert_eq!(player.position(), 85.0);
            player.skip(-1000.0);
            assert_eq!(player.position(), 0.0);
            player.skip(1.0e9);
            assert_eq!(player.position(), f64::from(episode.duration_seconds));
            player.seek(f64::NAN);
            assert_eq!(player.position(), 0.0);
            assert!(backend.calls().contains(&"seek 85".to_string()));
        });
    }

    #[test]
    fn plays_from_highlight() {
        with_owner(|| {
            let (player, backend) = setup();
            let episode = &EPISODES[0];
            player.play_from(episode, 120.0);
            assert_eq!(player.position(), 120.0);
            player.play_from(episode, 30.0);
            assert_eq!(player.position(), 30.0);
            let calls = backend.calls();
            assert_eq!(calls[2], "seek 120");
            assert_eq!(&calls[3..], ["play", "seek 30", "play"]);
        });
    }

    #[test]
    fn handles_backend_events() {
        with_owner(|| {
            let (player, _backend) = setup();
            assert_eq!(player.progress(), 0.0);
            player.play_episode(&EPISODES[0]);
            player.on_duration(200.0);
            player.on_duration(f64::INFINITY);
            player.on_duration(0.0);
            assert_eq!(player.duration(), 200.0);
            player.on_time_update(50.0);
            player.on_time_update(f64::NAN);
            assert_eq!(player.progress(), 0.25);
            player.on_play();
            player.on_ended();
            assert!(!player.is_playing());
            assert_eq!(player.position(), 0.0);
        });
    }

    #[test]
    fn episode_change_hook_is_optional() {
        struct Minimal;
        impl AudioBackend for Minimal {
            fn load(&self, _: &str) {}
            fn play(&self) {}
            fn pause(&self) {}
            fn seek(&self, _: f64) {}
        }
        with_owner(|| {
            let player = PlayerState::new();
            player.attach(Rc::new(Minimal));
            player.play_from(&EPISODES[0], 5.0);
            player.pause();
            assert_eq!(player.position(), 5.0);
        });
    }

    #[test]
    fn works_without_backend() {
        with_owner(|| {
            let player = PlayerState::default();
            player.play_episode(&EPISODES[0]);
            player.seek(10.0);
            player.pause();
            assert_eq!(player.position(), 10.0);
        });
    }

    #[test]
    fn snapshots_and_restores() {
        with_owner(|| {
            let (player, backend) = setup();
            assert_eq!(player.snapshot(), None);
            let episode = &EPISODES[0];
            player.play_episode(episode);
            player.on_time_update(123.4);
            let snapshot = player.snapshot().unwrap();
            assert_eq!(snapshot, format!("{}@120", episode.slug));

            let (restored, restored_backend) = setup();
            assert!(restored.restore(&snapshot));
            assert_eq!(restored.current(), Some(episode));
            assert_eq!(restored.position(), 120.0);
            assert!(!restored.is_playing());
            assert!(!restored_backend.calls().contains(&"play".to_string()));
            assert!(!restored.restore("nope@1"));
            assert!(!restored.restore("garbage"));
            assert!(backend.calls().len() >= 3);
        });
    }

    #[test]
    fn parses_snapshots() {
        let slug = EPISODES[0].slug;
        assert!(parse_snapshot(&format!("{slug}@15")).is_some());
        assert!(parse_snapshot(&format!("{slug}@-1")).is_none());
        assert!(parse_snapshot(&format!("{slug}@inf")).is_none());
        assert!(parse_snapshot(&format!("{slug}@x")).is_none());
        assert!(parse_snapshot("unknown@1").is_none());
        assert!(parse_snapshot("noseparator").is_none());
    }
}
