//! Episode teaser for the episode grid: image, title, date and summary.

use juoksevajohtaja_content::model::asset_path;
use leptos::prelude::*;

use crate::data::Episode;

#[component]
pub fn EpisodeCard(episode: &'static Episode) -> impl IntoView {
    view! {
        <article class="episode-card">
            <a class="episode-card__link" href=episode.path()>
                <img
                    class="episode-card__image"
                    src=asset_path(episode.image)
                    alt=episode.image_alt
                    width="600"
                    height="600"
                    loading="lazy"
                    decoding="async"
                />
                <div class="episode-card__body">
                    <p class="episode-card__meta">
                        <span>{episode.code()}</span>
                        <time datetime=episode.date.iso()>{episode.date.finnish()}</time>
                    </p>
                    <h3 class="episode-card__title">{episode.title}</h3>
                    <p class="episode-card__summary">{episode.summary_text}</p>
                </div>
            </a>
        </article>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::EPISODES;
    use crate::test_support::render;

    #[test]
    fn renders_card() {
        let episode = &EPISODES[0];
        let html = render("/", move || view! { <EpisodeCard episode=episode/> });
        assert!(html.contains(&format!("href=\"/jaksot/{}\"", episode.slug)));
        assert!(html.contains(&format!("src=\"{}\"", asset_path(episode.image))));
        assert!(html.contains("loading=\"lazy\""));
        assert!(html.contains(&format!("datetime=\"{}\"", episode.date.iso())));
        assert!(html.contains(&episode.date.finnish()));
        assert!(html.contains(episode.title));
        assert!(html.contains(&episode.code()));
    }
}
