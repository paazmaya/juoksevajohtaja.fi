//! Episode highlights. Timestamped highlights start the episode at that moment.

use juoksevajohtaja_content::time::clock;
use leptos::either::Either;
use leptos::prelude::*;

use crate::data::Episode;
use crate::player::PlayerState;

/// Click handler: play the episode from a highlight.
pub fn seek_handler<E>(
    player: PlayerState,
    episode: &'static Episode,
    seconds: u32,
) -> impl Fn(E) + 'static {
    move |_| player.play_from(episode, f64::from(seconds))
}

#[component]
pub fn HighlightList(episode: &'static Episode) -> impl IntoView {
    if episode.highlights.is_empty() {
        return None;
    }
    let player = expect_context::<PlayerState>();
    let items = episode
        .highlights
        .iter()
        .map(|highlight| {
            let marker = match highlight.time_seconds {
                Some(seconds) => Either::Left(view! {
                    <button
                        type="button"
                        class="highlight-list__time"
                        aria-label=format!("Kuuntele kohdasta {}", clock(seconds))
                        on:click=seek_handler(player, episode, seconds)
                    >
                        {clock(seconds)}
                    </button>
                }),
                None => Either::Right(
                    view! { <span class="highlight-list__bullet" aria-hidden="true"></span> },
                ),
            };
            view! {
                <li class="highlight-list__item">
                    {marker}
                    <span class="highlight-list__text">{highlight.text}</span>
                </li>
            }
        })
        .collect_view();
    Some(view! {
        <section class="highlight-list" aria-labelledby="highlights-title">
            <h2 class="highlight-list__title" id="highlights-title">"Jakson kohokohdat"</h2>
            <ol class="highlight-list__items" role="list">{items}</ol>
        </section>
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{Highlight, EPISODES};
    use crate::test_support::{provide_mock_player, render};

    #[test]
    fn renders_timestamped_and_plain_highlights() {
        static HIGHLIGHTS: [Highlight; 2] = [
            Highlight {
                time_seconds: Some(754),
                text: "Aikaleimalla",
            },
            Highlight {
                time_seconds: None,
                text: "Ilman aikaa",
            },
        ];
        let episode: &'static Episode = Box::leak(Box::new(Episode {
            highlights: &HIGHLIGHTS,
            ..EPISODES[0].clone()
        }));
        let html = render("/", move || {
            provide_mock_player();
            view! { <HighlightList episode=episode/> }
        });
        assert!(html.contains("Jakson kohokohdat"));
        assert!(html.contains("aria-label=\"Kuuntele kohdasta 12:34\""));
        assert!(html.contains("highlight-list__bullet"));
        assert!(html.contains("Ilman aikaa"));
    }

    #[test]
    fn click_plays_from_highlight() {
        crate::test_support::with_owner(|| {
            let (player, _) = provide_mock_player();
            seek_handler(player, &EPISODES[0], 90)(());
            assert_eq!(player.current(), Some(&EPISODES[0]));
            assert_eq!(player.position(), 90.0);
        });
    }

    #[test]
    fn renders_nothing_without_highlights() {
        let episode: &'static Episode = Box::leak(Box::new(Episode {
            highlights: &[],
            ..EPISODES[0].clone()
        }));
        let html = render("/", move || view! { <HighlightList episode=episode/> });
        assert!(!html.contains("highlight-list"));
    }
}
