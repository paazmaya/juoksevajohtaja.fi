//! Buttons and button-styled links.

use leptos::prelude::*;

use crate::components::icon::{Icon, IconName};

/// Visual weight of a button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
}

impl ButtonVariant {
    /// CSS classes for this variant.
    pub fn class(self) -> &'static str {
        match self {
            Self::Primary => "button button--primary",
            Self::Secondary => "button button--secondary",
            Self::Ghost => "button button--ghost",
        }
    }
}

/// A link that looks like a button. `external` links bypass the client-side router.
#[component]
pub fn ButtonLink(
    #[prop(into)] href: String,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] icon: Option<IconName>,
    #[prop(optional)] external: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <a class=variant.class() href=href rel=external.then_some("external")>
            {icon.map(|name| view! { <Icon name=name/> })}
            <span class="button__label">{children()}</span>
        </a>
    }
}

/// Click handler that runs a callback, generic over the event type so it can be unit tested.
pub fn run_on_click<E>(callback: Callback<()>) -> impl Fn(E) + 'static {
    move |_| callback.run(())
}

/// Icon-only button with an accessible label.
#[component]
pub fn IconButton(
    label: &'static str,
    #[prop(into)] icon: Signal<IconName>,
    on_click: Callback<()>,
    #[prop(optional)] variant: Option<ButtonVariant>,
) -> impl IntoView {
    let class = format!(
        "{} button--icon",
        variant.unwrap_or(ButtonVariant::Ghost).class()
    );
    view! {
        <button type="button" class=class aria-label=label title=label on:click=run_on_click(on_click)>
            {move || view! { <Icon name=icon.get()/> }}
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::render;

    #[test]
    fn maps_variants_to_classes() {
        assert_eq!(ButtonVariant::default(), ButtonVariant::Primary);
        assert_eq!(ButtonVariant::Primary.class(), "button button--primary");
        assert_eq!(ButtonVariant::Secondary.class(), "button button--secondary");
        assert_eq!(ButtonVariant::Ghost.class(), "button button--ghost");
    }

    #[test]
    fn renders_button_link() {
        let html = render("/", || {
            view! { <ButtonLink href="/peli" variant=ButtonVariant::Secondary icon=IconName::Game>"Pelaa"</ButtonLink> }
        });
        assert!(html.contains("href=\"/peli\""));
        assert!(html.contains("button--secondary"));
        assert!(html.contains(IconName::Game.path()));
        assert!(html.contains("Pelaa"));
        assert!(!html.contains("rel="));

        let html = render(
            "/",
            || view! { <ButtonLink href="/feed.xml" external=true>"RSS"</ButtonLink> },
        );
        assert!(html.contains("rel=\"external\""));
        assert!(html.contains("button--primary"));
    }

    #[test]
    fn click_runs_callback() {
        crate::test_support::with_owner(|| {
            let clicks = RwSignal::new(0);
            run_on_click(Callback::new(move |_| clicks.update(|n| *n += 1)))(());
            assert_eq!(clicks.get(), 1);
        });
    }

    #[test]
    fn renders_icon_button() {
        let html = render("/", || {
            view! { <IconButton label="Taaksepäin" icon=IconName::Back on_click=Callback::new(|_| {})/> }
        });
        assert!(html.contains("aria-label=\"Taaksepäin\""));
        assert!(html.contains("button--ghost button--icon"));
        assert!(html.contains(IconName::Back.path()));
    }
}
