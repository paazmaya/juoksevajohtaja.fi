//! Podcast catalogue compiled in from `episodes.yaml` by `build.rs`.

pub use juoksevajohtaja_content::{Episode, Guest, Highlight, Podcast};

include!(concat!(env!("OUT_DIR"), "/episodes.rs"));

/// Episode with the given slug.
pub fn episode_by_slug(slug: &str) -> Option<&'static Episode> {
    juoksevajohtaja_content::model::find_by_slug(EPISODES, slug)
}

/// Newest episode, if any.
pub fn latest_episode() -> Option<&'static Episode> {
    EPISODES.first()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_generated_catalogue() {
        assert_eq!(PODCAST.title, "Juokseva Johtaja");
        assert!(!EPISODES.is_empty());
        let latest = latest_episode().unwrap();
        assert_eq!(episode_by_slug(latest.slug), Some(latest));
        assert!(episode_by_slug("no-such-episode").is_none());
    }

    #[test]
    fn episodes_are_newest_first() {
        let dates: Vec<_> = EPISODES.iter().map(|episode| episode.date).collect();
        let mut newest_first = dates.clone();
        newest_first.sort();
        newest_first.reverse();
        assert_eq!(dates, newest_first);
    }
}
