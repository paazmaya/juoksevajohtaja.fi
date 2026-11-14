//! Responsive grid of episode cards.
//!
//! The catalogue is static, so the list is rendered once with a plain iterator instead of a reactive `<For>`.

use leptos::either::Either;
use leptos::prelude::*;

use crate::components::episode_card::EpisodeCard;
use crate::data::Episode;

#[component]
pub fn EpisodeGrid(episodes: &'static [Episode]) -> impl IntoView {
    if episodes.is_empty() {
        Either::Left(
            view! { <p class="episode-grid__empty">"Jaksoja ei ole vielä julkaistu. Pysy kuulolla!"</p> },
        )
    } else {
        Either::Right(view! {
            <ul class="episode-grid" role="list">
                {episodes
                    .iter()
                    .map(|episode| view! { <li class="episode-grid__item"><EpisodeCard episode=episode/></li> })
                    .collect_view()}
            </ul>
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::EPISODES;
    use crate::test_support::render;

    #[test]
    fn renders_all_episodes() {
        let html = render("/", || view! { <EpisodeGrid episodes=EPISODES/> });
        assert_eq!(
            html.matches("class=\"episode-grid__item\"").count(),
            EPISODES.len()
        );
    }

    #[test]
    fn renders_empty_state() {
        let html = render("/", || view! { <EpisodeGrid episodes=&[]/> });
        assert!(html.contains("episode-grid__empty"));
        assert!(!html.contains("<ul"));
    }
}
