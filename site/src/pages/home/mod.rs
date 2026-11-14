//! Front page: podcast pitch and the episode grid.

use juoksevajohtaja_content::seo::{self, FEED_PATH, GAME_PATH};
use leptos::prelude::*;

use crate::components::{ButtonLink, ButtonVariant, EpisodeGrid, IconName, PageHead, PlayButton};
use crate::data::{latest_episode, EPISODES, PODCAST};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <PageHead meta=seo::home(&PODCAST)/>
        <section class="home-hero">
            <div class="home-hero__inner">
                <p class="home-hero__eyebrow">"Podcast johtajuudesta juoksulenkillä"</p>
                <h1 class="home-hero__title">{PODCAST.title}</h1>
                <p class="home-hero__lead">{PODCAST.description}</p>
                <div class="home-hero__actions">
                    {latest_episode().map(|episode| view! { <PlayButton episode=episode/> })}
                    <ButtonLink href=FEED_PATH variant=ButtonVariant::Secondary icon=IconName::Rss external=true>
                        "Tilaa RSS-syöte"
                    </ButtonLink>
                    <ButtonLink href=GAME_PATH variant=ButtonVariant::Ghost icon=IconName::Game>
                        "Pelaa juoksupeliä"
                    </ButtonLink>
                </div>
            </div>
        </section>
        <section class="home-episodes" aria-labelledby="episodes-title">
            <h2 class="home-episodes__title" id="episodes-title">"Jaksot"</h2>
            <EpisodeGrid episodes=EPISODES/>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{provide_mock_player, render};

    #[test]
    fn renders_hero_and_grid() {
        let html = render("/", || {
            provide_mock_player();
            view! { <HomePage/> }
        });
        assert!(html.contains("<h1 class=\"home-hero__title\">Juokseva Johtaja</h1>"));
        assert!(html.contains("Kuuntele jakso"));
        assert!(html.contains("href=\"/feed.xml\""));
        assert!(html.contains("href=\"/peli\""));
        assert_eq!(
            html.matches("class=\"episode-card\"").count(),
            EPISODES.len()
        );
    }
}
