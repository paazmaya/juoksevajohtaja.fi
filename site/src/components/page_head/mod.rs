//! Document `<head>` for a page (title, description, canonical, Open Graph, JSON-LD) via `leptos_meta`.
//!
//! Uses the same [`PageMeta`] as the pre-renderer, so crawlers that run JavaScript see identical tags.

use juoksevajohtaja_content::seo::{PageMeta, FEED_PATH};
use leptos::prelude::*;
use leptos_meta::{Link, Meta, Script, Title};

use crate::data::PODCAST;

#[component]
pub fn PageHead(meta: PageMeta) -> impl IntoView {
    let canonical = meta.canonical_url(&PODCAST);
    let image = meta.image_url(&PODCAST);
    let PageMeta {
        title,
        description,
        image_alt,
        og_type,
        json_ld,
        noindex,
        ..
    } = meta;
    view! {
        <Title text=title.clone()/>
        <Meta name="description" content=description.clone()/>
        {noindex.then(|| view! { <Meta name="robots" content="noindex"/> })}
        {(!noindex).then(|| view! { <Link rel="canonical" href=canonical.clone()/> })}
        <Link rel="alternate" type_="application/rss+xml" title=PODCAST.title href=FEED_PATH/>
        <Meta property="og:type" content=og_type/>
        <Meta property="og:site_name" content=PODCAST.title/>
        <Meta property="og:title" content=title.clone()/>
        <Meta property="og:description" content=description.clone()/>
        <Meta property="og:url" content=canonical/>
        <Meta property="og:image" content=image.clone()/>
        <Meta property="og:image:alt" content=image_alt/>
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content=title/>
        <Meta name="twitter:description" content=description/>
        <Meta name="twitter:image" content=image/>
        {json_ld.map(|json| view! { <Script type_="application/ld+json">{json}</Script> })}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::EPISODES;
    use crate::test_support::render_with_head;
    use juoksevajohtaja_content::seo;

    #[test]
    fn renders_episode_head() {
        let meta = seo::episode(&PODCAST, &EPISODES[0]);
        let expected_title = meta.title.clone();
        let (_, head) = render_with_head("/", move || view! { <PageHead meta=meta/> });
        assert!(head.contains(&format!("<title>{}", expected_title)));
        assert!(head.contains("rel=\"canonical\""));
        assert!(head.contains("og:image"));
        assert!(head.contains("application/ld+json"));
        assert!(!head.contains("noindex"));
    }

    #[test]
    fn renders_noindex_head() {
        let meta = seo::not_found(&PODCAST);
        let (_, head) = render_with_head("/", move || view! { <PageHead meta=meta/> });
        assert!(head.contains("noindex"));
        assert!(!head.contains("canonical"));
        assert!(!head.contains("ld+json"));
    }
}
