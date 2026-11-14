//! Inline SVG icons that inherit the current text color.

use leptos::prelude::*;

/// Available icons (24×24 grid).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconName {
    Play,
    Pause,
    Back,
    Forward,
    Rss,
    Game,
    Headphones,
    ArrowLeft,
}

impl IconName {
    /// SVG path data.
    pub fn path(self) -> &'static str {
        match self {
            Self::Play => "M8 5v14l11-7z",
            Self::Pause => "M6 5h4v14H6zM14 5h4v14h-4z",
            Self::Back => "M11 18V6l-8.5 6zm.5-6 8.5 6V6z",
            Self::Forward => "M4 18l8.5-6L4 6zm9-12v12l8.5-6z",
            Self::Rss => "M6.2 15.6a2.2 2.2 0 1 1 0 4.4 2.2 2.2 0 0 1 0-4.4zM4 4.4A15.6 15.6 0 0 1 19.6 20h-2.9A12.7 12.7 0 0 0 4 7.3zm0 5.7A9.9 9.9 0 0 1 13.9 20h-2.8A7.1 7.1 0 0 0 4 12.9z",
            Self::Game => "M7 7h10a5 5 0 0 1 0 10c-1.6 0-2.6-.8-3.4-2h-3.2C9.6 16.2 8.6 17 7 17A5 5 0 0 1 7 7zm0 3v1.5H5.5v1H7V14h1v-1.5h1.5v-1H8V10zm8.5 1a1 1 0 1 0 0 2 1 1 0 0 0 0-2zm2-2a1 1 0 1 0 0 2 1 1 0 0 0 0-2z",
            Self::Headphones => "M12 3a9 9 0 0 0-9 9v7a2 2 0 0 0 2 2h2v-8H5v-1a7 7 0 0 1 14 0v1h-2v8h2a2 2 0 0 0 2-2v-7a9 9 0 0 0-9-9z",
            Self::ArrowLeft => "M20 11H7.8l5.6-5.6L12 4l-8 8 8 8 1.4-1.4L7.8 13H20z",
        }
    }
}

/// Decorative icon; give the surrounding control an accessible label.
#[component]
pub fn Icon(name: IconName) -> impl IntoView {
    view! {
        <svg class="icon" viewBox="0 0 24 24" aria-hidden="true" focusable="false">
            <path d=name.path()></path>
        </svg>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::render;

    #[test]
    fn renders_every_icon() {
        for name in [
            IconName::Play,
            IconName::Pause,
            IconName::Back,
            IconName::Forward,
            IconName::Rss,
            IconName::Game,
            IconName::Headphones,
            IconName::ArrowLeft,
        ] {
            let html = render("/", move || view! { <Icon name=name/> });
            assert!(html.contains("class=\"icon\""));
            assert!(html.contains("aria-hidden=\"true\""));
            assert!(html.contains(name.path()));
        }
    }
}
