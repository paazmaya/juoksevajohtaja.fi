//! The interviewed person: name, role, photo (or initials) and introduction.

use juoksevajohtaja_content::model::asset_path;
use leptos::either::Either;
use leptos::prelude::*;

use crate::data::Guest;

/// Up to two initials for the avatar placeholder: `Anna Maria Virtanen` → `AV`.
pub fn initials(name: &str) -> String {
    let words: Vec<&str> = name.split_whitespace().collect();
    let pick = |word: &str| {
        word.chars()
            .next()
            .map(|c| c.to_uppercase().collect::<String>())
            .unwrap_or_default()
    };
    match words.as_slice() {
        [] => "?".to_string(),
        [only] => pick(only),
        [first, .., last] => pick(first) + &pick(last),
    }
}

#[component]
pub fn GuestCard(guest: &'static Guest, intro_html: &'static str) -> impl IntoView {
    let avatar = match guest.image {
        Some(image) => Either::Left(view! {
            <img class="guest-card__avatar" src=asset_path(image) alt="" width="160" height="160" loading="lazy"/>
        }),
        None => Either::Right(view! {
            <span class="guest-card__avatar guest-card__avatar--initials" aria-hidden="true">{initials(guest.name)}</span>
        }),
    };
    view! {
        <section class="guest-card" aria-labelledby="guest-name">
            <div class="guest-card__header">
                {avatar}
                <div>
                    <p class="guest-card__label">"Vieras"</p>
                    <h2 class="guest-card__name" id="guest-name">{guest.name}</h2>
                    <p class="guest-card__role">{guest.role}</p>
                </div>
            </div>
            <div class="guest-card__intro prose" inner_html=intro_html></div>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::render;

    static WITH_IMAGE: Guest = Guest {
        name: "Anna Virtanen",
        role: "CTO",
        image: Some("images/anna.png"),
    };
    static WITHOUT_IMAGE: Guest = Guest {
        name: "Mystery",
        role: "VP",
        image: None,
    };

    #[test]
    fn computes_initials() {
        assert_eq!(initials("Anna Maria Virtanen"), "AV");
        assert_eq!(initials("ärjä"), "Ä");
        assert_eq!(initials("   "), "?");
    }

    #[test]
    fn renders_photo() {
        let html = render(
            "/",
            || view! { <GuestCard guest=&WITH_IMAGE intro_html="<p>Esittely</p>"/> },
        );
        assert!(html.contains("src=\"/images/anna.png\""));
        assert!(html.contains("Anna Virtanen"));
        assert!(html.contains("CTO"));
        assert!(html.contains("<p>Esittely</p>"));
    }

    #[test]
    fn renders_initials_without_photo() {
        let html = render(
            "/",
            || view! { <GuestCard guest=&WITHOUT_IMAGE intro_html=""/> },
        );
        assert!(html.contains("guest-card__avatar--initials"));
        assert!(html.contains(">M<"));
        assert!(!html.contains("<img"));
    }
}
