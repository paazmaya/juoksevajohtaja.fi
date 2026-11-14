//! Podcast RSS feed, sitemap and robots.txt.

use crate::escape::xml;
use crate::model::{absolute_url, Episode, Podcast};
use crate::seo::{FEED_PATH, GAME_PATH};
use crate::time::clock;

/// MIME type of an audio enclosure, derived from the URL extension.
pub fn audio_mime_type(url: &str) -> &'static str {
    let path = url.split(['?', '#']).next().unwrap_or(url).to_lowercase();
    if path.ends_with(".m4a") || path.ends_with(".mp4") || path.ends_with(".aac") {
        "audio/mp4"
    } else if path.ends_with(".ogg") || path.ends_with(".oga") || path.ends_with(".opus") {
        "audio/ogg"
    } else {
        "audio/mpeg"
    }
}

/// RSS 2.0 feed with Apple Podcasts (`itunes:`) tags. Episodes are written in the given order.
pub fn rss(podcast: &Podcast, episodes: &[Episode]) -> String {
    let site_url = absolute_url(podcast.base_url, "/");
    let mut out = String::new();
    out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    out.push_str(
        "<rss version=\"2.0\" xmlns:itunes=\"http://www.itunes.com/dtds/podcast-1.0.dtd\" \
         xmlns:atom=\"http://www.w3.org/2005/Atom\" xmlns:content=\"http://purl.org/rss/1.0/modules/content/\">\n",
    );
    out.push_str("<channel>\n");
    out.push_str(&format!("<title>{}</title>\n", xml(podcast.title)));
    out.push_str(&format!("<link>{}</link>\n", xml(&site_url)));
    out.push_str(&format!(
        "<atom:link href=\"{}\" rel=\"self\" type=\"application/rss+xml\"/>\n",
        xml(&absolute_url(podcast.base_url, FEED_PATH))
    ));
    out.push_str(&format!(
        "<description>{}</description>\n",
        xml(podcast.description)
    ));
    out.push_str(&format!("<language>{}</language>\n", xml(podcast.language)));
    out.push_str(&format!(
        "<itunes:author>{}</itunes:author>\n",
        xml(podcast.author)
    ));
    out.push_str(&format!(
        "<itunes:summary>{}</itunes:summary>\n",
        xml(podcast.description)
    ));
    out.push_str(&format!(
        "<itunes:image href=\"{}\"/>\n",
        xml(&absolute_url(podcast.base_url, podcast.image))
    ));
    out.push_str("<itunes:explicit>false</itunes:explicit>\n");
    out.push_str("<itunes:type>episodic</itunes:type>\n");
    if let Some(email) = podcast.email {
        out.push_str(&format!(
            "<itunes:owner><itunes:name>{}</itunes:name><itunes:email>{}</itunes:email></itunes:owner>\n",
            xml(podcast.author),
            xml(email)
        ));
    }
    for category in podcast.categories {
        out.push_str(&format!("<itunes:category text=\"{}\"/>\n", xml(category)));
    }
    if let Some(latest) = episodes.iter().map(|e| e.date).max() {
        out.push_str(&format!(
            "<lastBuildDate>{}</lastBuildDate>\n",
            latest.rfc2822()
        ));
    }
    for episode in episodes {
        let page = absolute_url(podcast.base_url, &episode.path());
        out.push_str("<item>\n");
        out.push_str(&format!("<title>{}</title>\n", xml(episode.title)));
        out.push_str(&format!("<link>{}</link>\n", xml(&page)));
        out.push_str(&format!(
            "<guid isPermaLink=\"true\">{}</guid>\n",
            xml(&page)
        ));
        out.push_str(&format!("<pubDate>{}</pubDate>\n", episode.date.rfc2822()));
        out.push_str(&format!(
            "<description>{}</description>\n",
            xml(episode.summary_text)
        ));
        out.push_str(&format!(
            "<content:encoded><![CDATA[{}{}]]></content:encoded>\n",
            episode.summary_html.replace("]]>", "]]]]><![CDATA[>"),
            episode.intro_html.replace("]]>", "]]]]><![CDATA[>")
        ));
        out.push_str(&format!(
            "<enclosure url=\"{}\" length=\"{}\" type=\"{}\"/>\n",
            xml(episode.audio_url),
            episode.audio_bytes,
            audio_mime_type(episode.audio_url)
        ));
        out.push_str(&format!(
            "<itunes:duration>{}</itunes:duration>\n",
            clock(episode.duration_seconds)
        ));
        out.push_str(&format!(
            "<itunes:season>{}</itunes:season>\n",
            episode.season
        ));
        out.push_str(&format!(
            "<itunes:episode>{}</itunes:episode>\n",
            episode.number
        ));
        out.push_str(&format!(
            "<itunes:image href=\"{}\"/>\n",
            xml(&absolute_url(podcast.base_url, episode.image))
        ));
        out.push_str("<itunes:episodeType>full</itunes:episodeType>\n");
        out.push_str("</item>\n");
    }
    out.push_str("</channel>\n</rss>\n");
    out
}

/// XML sitemap with home, game and every episode page.
pub fn sitemap(podcast: &Podcast, episodes: &[Episode]) -> String {
    let mut out = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
    );
    let latest = episodes.iter().map(|e| e.date).max();
    let mut push = |path: &str, lastmod: Option<String>| {
        out.push_str(&format!(
            "<url><loc>{}</loc>",
            xml(&absolute_url(podcast.base_url, path))
        ));
        if let Some(date) = lastmod {
            out.push_str(&format!("<lastmod>{date}</lastmod>"));
        }
        out.push_str("</url>\n");
    };
    push("/", latest.map(|d| d.iso()));
    for episode in episodes {
        push(&episode.path(), Some(episode.date.iso()));
    }
    push(GAME_PATH, None);
    out.push_str("</urlset>\n");
    out
}

/// robots.txt that allows everything except the embedded game build and points to the sitemap.
pub fn robots(podcast: &Podcast, disallow: &[&str]) -> String {
    let mut out = String::from("User-agent: *\n");
    for path in disallow {
        out.push_str(&format!("Disallow: {path}\n"));
    }
    out.push_str(&format!(
        "\nSitemap: {}\n",
        absolute_url(podcast.base_url, "/sitemap.xml")
    ));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::fixtures::{episode, podcast};

    #[test]
    fn detects_mime_types() {
        assert_eq!(audio_mime_type("https://a/b.MP3?x=1"), "audio/mpeg");
        assert_eq!(audio_mime_type("https://a/b.m4a"), "audio/mp4");
        assert_eq!(audio_mime_type("https://a/b.opus#t"), "audio/ogg");
        assert_eq!(audio_mime_type("https://a/b"), "audio/mpeg");
    }

    #[test]
    fn renders_rss() {
        let mut ep = episode();
        ep.intro_html = "<p>a]]>b</p>";
        let feed = rss(&podcast(), &[ep]);
        assert!(feed.starts_with("<?xml"));
        assert!(feed.contains("<title>Juokseva Johtaja</title>"));
        assert!(feed.contains("Podcast &lt;johtajuudesta&gt;"));
        assert!(feed.contains("<itunes:email>podcast@example.com</itunes:email>"));
        assert!(feed.contains("<itunes:category text=\"Business\"/>"));
        assert!(feed.contains("<pubDate>Thu, 15 Jan 2026 00:00:00 +0000</pubDate>"));
        assert!(feed.contains("<enclosure url=\"https://cdn.example.com/e2.mp3\" length=\"1234\" type=\"audio/mpeg\"/>"));
        assert!(feed.contains("<itunes:duration>45:12</itunes:duration>"));
        assert!(feed
            .contains("<guid isPermaLink=\"true\">https://juoksevajohtaja.fi/jaksot/testi</guid>"));
        assert!(feed.contains("a]]]]><![CDATA[>b"));
        assert!(feed.contains("<lastBuildDate>"));
    }

    #[test]
    fn renders_rss_without_owner_or_episodes() {
        let mut podcast = podcast();
        podcast.email = None;
        let feed = rss(&podcast, &[]);
        assert!(!feed.contains("itunes:owner"));
        assert!(!feed.contains("<item>"));
        assert!(!feed.contains("lastBuildDate"));
    }

    #[test]
    fn renders_sitemap() {
        let map = sitemap(&podcast(), &[episode()]);
        assert!(map.contains(
            "<url><loc>https://juoksevajohtaja.fi/</loc><lastmod>2026-01-15</lastmod></url>"
        ));
        assert!(map.contains("<loc>https://juoksevajohtaja.fi/jaksot/testi</loc>"));
        assert!(map.contains("<url><loc>https://juoksevajohtaja.fi/peli</loc></url>"));
        let empty = sitemap(&podcast(), &[]);
        assert!(empty.contains("<url><loc>https://juoksevajohtaja.fi/</loc></url>"));
    }

    #[test]
    fn renders_robots() {
        let robots = robots(&podcast(), &["/peli-app/"]);
        assert_eq!(
            robots,
            "User-agent: *\nDisallow: /peli-app/\n\nSitemap: https://juoksevajohtaja.fi/sitemap.xml\n"
        );
    }
}
