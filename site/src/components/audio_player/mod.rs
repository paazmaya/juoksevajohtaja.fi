//! Persistent bottom audio player. Rendered once outside `<Routes>`, so playback continues across navigation.

use juoksevajohtaja_content::model::asset_path;
use juoksevajohtaja_content::time::clock;
use leptos::prelude::*;

use crate::components::button::{ButtonVariant, IconButton};
use crate::components::icon::IconName;
use crate::player::{PlayerState, SKIP_BACK_SECONDS, SKIP_FORWARD_SECONDS};
use crate::web::{self, DomEvent};

/// Whole seconds for display, robust against NaN/negative values from media elements.
pub fn display_seconds(value: f64) -> u32 {
    if value.is_finite() && value > 0.0 {
        value.floor() as u32
    } else {
        0
    }
}

/// One transport control button.
pub struct Control {
    pub label: &'static str,
    pub icon: Signal<IconName>,
    pub variant: ButtonVariant,
    pub action: Callback<()>,
}

/// Back, play/pause and forward controls, in display order.
pub fn controls(player: PlayerState) -> [Control; 3] {
    [
        Control {
            label: "15 sekuntia taaksepäin",
            icon: Signal::stored(IconName::Back),
            variant: ButtonVariant::Ghost,
            action: Callback::new(move |_| player.skip(-SKIP_BACK_SECONDS)),
        },
        Control {
            label: "Toista tai tauko",
            icon: Signal::derive(move || {
                if player.is_playing() {
                    IconName::Pause
                } else {
                    IconName::Play
                }
            }),
            variant: ButtonVariant::Primary,
            action: Callback::new(move |_| player.toggle()),
        },
        Control {
            label: "30 sekuntia eteenpäin",
            icon: Signal::stored(IconName::Forward),
            variant: ButtonVariant::Ghost,
            action: Callback::new(move |_| player.skip(SKIP_FORWARD_SECONDS)),
        },
    ]
}

/// `<audio>` and progress-bar event handlers, generic over the event so they can be unit tested.
pub mod handlers {
    use super::*;

    pub fn play<E>(player: PlayerState) -> impl Fn(E) + 'static {
        move |_| player.on_play()
    }

    pub fn pause<E>(player: PlayerState) -> impl Fn(E) + 'static {
        move |_| player.on_pause()
    }

    pub fn ended<E>(player: PlayerState) -> impl Fn(E) + 'static {
        move |_| player.on_ended()
    }

    pub fn time_update<E: DomEvent>(player: PlayerState) -> impl Fn(E) + 'static {
        move |event| player.on_time_update(event.media_times().0)
    }

    pub fn duration_change<E: DomEvent>(player: PlayerState) -> impl Fn(E) + 'static {
        move |event| player.on_duration(event.media_times().1)
    }

    pub fn seek<E: DomEvent>(player: PlayerState) -> impl Fn(E) + 'static {
        move |event| player.seek(event.input_value().parse().unwrap_or(0.0))
    }
}

#[component]
pub fn AudioPlayer() -> impl IntoView {
    let player = expect_context::<PlayerState>();
    let audio = NodeRef::<leptos::html::Audio>::new();
    web::bind_audio(player, audio);

    let time_label = move || {
        format!(
            "{} / {}",
            clock(display_seconds(player.position())),
            clock(display_seconds(player.duration()))
        )
    };

    let bar = move || {
        player.current().map(|episode| {
            let buttons = controls(player)
                .into_iter()
                .map(|control| {
                    view! {
                        <IconButton
                            label=control.label
                            icon=control.icon
                            variant=control.variant
                            on_click=control.action
                        />
                    }
                })
                .collect_view();
            view! {
                <div class="audio-player__spacer" aria-hidden="true"></div>
                <div class="audio-player__bar">
                    <img class="audio-player__cover" src=asset_path(episode.image) alt="" width="56" height="56"/>
                    <div class="audio-player__info">
                        <a class="audio-player__title" href=episode.path()>{episode.title}</a>
                        <span class="audio-player__time">{time_label}</span>
                    </div>
                    <div class="audio-player__controls">{buttons}</div>
                    <input
                        class="audio-player__progress"
                        type="range"
                        min="0"
                        step="1"
                        aria-label="Kohta jaksossa"
                        prop:max=move || display_seconds(player.duration())
                        prop:value=move || display_seconds(player.position())
                        on:change=handlers::seek(player)
                    />
                </div>
            }
        })
    };

    view! {
        <section class="audio-player" aria-label="Podcast-soitin">
            <audio
                node_ref=audio
                preload="metadata"
                on:play=handlers::play(player)
                on:pause=handlers::pause(player)
                on:ended=handlers::ended(player)
                on:timeupdate=handlers::time_update(player)
                on:durationchange=handlers::duration_change(player)
            ></audio>
            {bar}
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::EPISODES;
    use crate::test_support::{provide_mock_player, render, with_owner, FakeEvent};

    #[test]
    fn converts_display_seconds() {
        assert_eq!(display_seconds(12.9), 12);
        assert_eq!(display_seconds(-1.0), 0);
        assert_eq!(display_seconds(f64::NAN), 0);
    }

    #[test]
    fn hides_bar_when_idle() {
        let html = render("/", || {
            provide_mock_player();
            view! { <AudioPlayer/> }
        });
        assert!(html.contains("<audio"));
        assert!(!html.contains("audio-player__bar"));
    }

    #[test]
    fn shows_current_episode() {
        let episode = &EPISODES[0];
        let html = render("/", move || {
            let (player, _) = provide_mock_player();
            player.play_from(episode, 75.0);
            player.on_play();
            view! { <AudioPlayer/> }
        });
        assert!(html.contains("audio-player__bar"));
        assert!(html.contains(episode.title));
        assert!(html.contains(&format!("href=\"{}\"", episode.path())));
        assert!(html.contains(&format!("1:15 / {}", clock(episode.duration_seconds))));
        assert!(html.contains("aria-label=\"15 sekuntia taaksepäin\""));
        assert!(html.contains(IconName::Pause.path()));
    }

    #[test]
    fn controls_drive_the_player() {
        with_owner(|| {
            let (player, backend) = provide_mock_player();
            player.play_from(&EPISODES[0], 100.0);
            let [back, toggle, forward] = controls(player);
            assert_eq!(toggle.icon.get(), IconName::Play);
            back.action.run(());
            assert_eq!(player.position(), 85.0);
            forward.action.run(());
            assert_eq!(player.position(), 115.0);
            player.on_play();
            assert_eq!(toggle.icon.get(), IconName::Pause);
            assert_eq!(toggle.variant, ButtonVariant::Primary);
            toggle.action.run(());
            assert_eq!(backend.calls().last().map(String::as_str), Some("pause"));
        });
    }

    #[test]
    fn media_events_update_state() {
        with_owner(|| {
            let (player, _) = provide_mock_player();
            player.play_episode(&EPISODES[0]);
            handlers::play(player)(());
            assert!(player.is_playing());
            handlers::duration_change(player)(FakeEvent::media(0.0, 600.0));
            assert_eq!(player.duration(), 600.0);
            handlers::time_update(player)(FakeEvent::media(42.0, 600.0));
            assert_eq!(player.position(), 42.0);
            handlers::seek(player)(FakeEvent::input("300"));
            assert_eq!(player.position(), 300.0);
            handlers::seek(player)(FakeEvent::input("bad"));
            assert_eq!(player.position(), 0.0);
            handlers::pause(player)(());
            assert!(!player.is_playing());
            handlers::play(player)(());
            handlers::ended(player)(());
            assert!(!player.is_playing());
        });
    }
}
