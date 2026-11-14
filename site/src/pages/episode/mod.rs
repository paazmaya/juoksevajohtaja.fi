//! Episode page: cover, title, date, introduction, guest and highlights.

use juoksevajohtaja_content::model::asset_path;
use juoksevajohtaja_content::seo;
use juoksevajohtaja_content::time::human_duration;
use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::{GuestCard, HighlightList, Icon, IconName, PageHead, PlayButton};
use crate::data::{episode_by_slug, Episode, PODCAST};
use crate::pages::not_found::NotFoundPage;

/// Resolves `/jaksot/:slug`; re-renders when navigating between episodes.
#[component]
pub fn EpisodeRoute() -> impl IntoView {
    let params = use_params_map();
    move || match params
        .read()
        .get("slug")
        .and_then(|slug| episode_by_slug(&slug))
    {
        Some(episode) => Either::Left(view! { <EpisodePage episode=episode/> }),
        None => Either::Right(view! { <NotFoundPage/> }),
    }
}

#[component]
pub fn EpisodePage(episode: &'static Episode) -> impl IntoView {
    view! {
        <PageHead meta=seo::episode(&PODCAST, episode)/>
        <article class="episode-page">
            <header class="episode-page__header">
                <img
                    class="episode-page__image"
                    src=asset_path(episode.image)
                    alt=episode.image_alt
                    width="600"
                    height="600"
                    fetchpriority="high"
                />
                <div class="episode-page__heading">
                    <p class="episode-page__meta">
                        <span>{episode.code()}</span>
                        <time datetime=episode.date.iso()>{episode.date.finnish()}</time>
                        <span>{human_duration(episode.duration_seconds)}</span>
                    </p>
                    <h1 class="episode-page__title">{episode.title}</h1>
                    <div class="episode-page__summary prose" inner_html=episode.summary_html></div>
                    <PlayButton episode=episode/>
                </div>
            </header>
            <div class="episode-page__content">
                <GuestCard guest=&episode.guest intro_html=episode.intro_html/>
                <HighlightList episode=episode/>
            </div>
            <a class="episode-page__back" href="/">
                <Icon name=IconName::ArrowLeft/>
                "Kaikki jaksot"
            </a>
        </article>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::EPISODES;
    use crate::test_support::{provide_mock_player, render};

    #[test]
    fn renders_episode_route() {
        let episode = &EPISODES[0];
        let html = render(&episode.path(), || {
            provide_mock_player();
            view! {
                <leptos_router::components::Routes fallback=|| "fallback">
                    <leptos_router::components::Route path=leptos_router::path!("/jaksot/:slug") view=EpisodeRoute/>
                </leptos_router::components::Routes>
            }
        });
        assert!(html.contains(&format!(
            "<h1 class=\"episode-page__title\">{}</h1>",
            episode.title
        )));
        assert!(html.contains(episode.summary_html));
        assert!(html.contains(episode.intro_html));
        assert!(html.contains(episode.guest.name));
        assert!(html.contains(&human_duration(episode.duration_seconds)));
        assert!(html.contains("Kuuntele jakso"));
    }

    #[test]
    fn renders_not_found_for_unknown_slug() {
        let html = render("/jaksot/ei-ole", || {
            provide_mock_player();
            view! {
                <leptos_router::components::Routes fallback=|| "fallback">
                    <leptos_router::components::Route path=leptos_router::path!("/jaksot/:slug") view=EpisodeRoute/>
                </leptos_router::components::Routes>
            }
        });
        assert!(html.contains("Sivua ei löytynyt"));
    }
}
