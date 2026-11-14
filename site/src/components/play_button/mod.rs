//! Play/pause button for one episode, connected to the site-wide player.

use leptos::prelude::*;

use crate::components::icon::{Icon, IconName};
use crate::data::Episode;
use crate::player::PlayerState;

/// Button label for the current state.
pub fn label(playing: bool) -> &'static str {
    if playing {
        "Tauko"
    } else {
        "Kuuntele jakso"
    }
}

/// Click handler: play/pause this episode.
pub fn toggle_handler<E>(player: PlayerState, episode: &'static Episode) -> impl Fn(E) + 'static {
    move |_| player.toggle_episode(episode)
}

#[component]
pub fn PlayButton(episode: &'static Episode) -> impl IntoView {
    let player = expect_context::<PlayerState>();
    let playing = Memo::new(move |_| player.is_playing_episode(episode));
    view! {
        <button
            type="button"
            class="button button--primary play-button"
            aria-pressed=move || playing.get().to_string()
            on:click=toggle_handler(player, episode)
        >
            {move || view! { <Icon name=if playing.get() { IconName::Pause } else { IconName::Play }/> }}
            <span class="play-button__label">{move || label(playing.get())}</span>
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::EPISODES;
    use crate::test_support::{provide_mock_player, render};

    #[test]
    fn labels_state() {
        assert_eq!(label(true), "Tauko");
        assert_eq!(label(false), "Kuuntele jakso");
    }

    #[test]
    fn click_toggles_episode() {
        crate::test_support::with_owner(|| {
            let (player, backend) = provide_mock_player();
            toggle_handler(player, &EPISODES[0])(());
            assert_eq!(player.current(), Some(&EPISODES[0]));
            assert_eq!(backend.calls().last().map(String::as_str), Some("play"));
        });
    }

    #[test]
    fn renders_idle_state() {
        let html = render("/", || {
            provide_mock_player();
            view! { <PlayButton episode=&EPISODES[0]/> }
        });
        assert!(html.contains("Kuuntele jakso"));
        assert!(html.contains("aria-pressed=\"false\""));
        assert!(html.contains(IconName::Play.path()));
    }

    #[test]
    fn renders_playing_state() {
        let html = render("/", || {
            let (player, _) = provide_mock_player();
            player.play_episode(&EPISODES[0]);
            player.on_play();
            view! { <PlayButton episode=&EPISODES[0]/> }
        });
        assert!(html.contains("Tauko"));
        assert!(html.contains("aria-pressed=\"true\""));
        assert!(html.contains(IconName::Pause.path()));
    }
}
