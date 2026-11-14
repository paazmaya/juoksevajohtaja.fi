//! Loading animation: the podcast host (headphones, orange shirt, navy shorts, blue shoes) running in place on
//! a moving ground line. Pure SVG + CSS, so it needs no extra request and can be pre-rendered into the page
//! shell to show while the WebAssembly bundle loads.

use leptos::prelude::*;

/// Where the loader is shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LoaderVariant {
    /// Slim strip at the bottom of the viewport, page content stays readable.
    #[default]
    Strip,
    /// Centered inside its container (e.g. the game frame).
    Overlay,
}

impl LoaderVariant {
    pub fn class(self) -> &'static str {
        match self {
            Self::Strip => "runner-loader runner-loader--strip",
            Self::Overlay => "runner-loader runner-loader--overlay",
        }
    }
}

/// The running host figure on its own, reusable anywhere.
#[component]
pub fn RunnerFigure() -> impl IntoView {
    view! {
        <svg class="runner" viewBox="0 0 64 64" aria-hidden="true" focusable="false">
            <g class="runner__body">
                <rect class="runner__limb runner__skin runner__arm runner__arm--back" x="33" y="22" width="4" height="13" rx="2"></rect>
                <g class="runner__leg runner__leg--back">
                    <rect class="runner__skin" x="33" y="40" width="5" height="13" rx="2.5"></rect>
                    <rect class="runner__shoe" x="32" y="51" width="10" height="4" rx="2"></rect>
                    <rect class="runner__sole" x="32" y="54" width="10" height="1.5" rx="0.75"></rect>
                </g>
                <rect class="runner__shirt" x="29" y="21" width="12" height="16" rx="5"></rect>
                <rect class="runner__shorts" x="29" y="34" width="12" height="8" rx="3"></rect>
                <g class="runner__leg runner__leg--front">
                    <rect class="runner__skin" x="33" y="40" width="5" height="13" rx="2.5"></rect>
                    <rect class="runner__shoe" x="32" y="51" width="10" height="4" rx="2"></rect>
                    <rect class="runner__sole" x="32" y="54" width="10" height="1.5" rx="0.75"></rect>
                </g>
                <rect class="runner__limb runner__skin runner__arm runner__arm--front" x="33" y="22" width="4" height="13" rx="2"></rect>
                <circle class="runner__skin" cx="36" cy="13" r="7"></circle>
                <path class="runner__hair" d="M29.2 12a7 7 0 0 1 13.4-2.6c-3 .4-6.6.2-9.4-1.4-.6 1.8-2 3.3-4 4z"></path>
                <path class="runner__hair" d="M37.5 17.8c1.8 1.6 4.6 1.4 5.3-.8.3 1.9-.6 3.6-2.4 4.3-1.5.6-2.6-1.2-2.9-3.5z"></path>
                <path class="runner__headphones" d="M29.5 12a6.5 6.5 0 0 1 13 0" fill="none"></path>
                <rect class="runner__earcup" x="33" y="10.5" width="4" height="6" rx="1.8"></rect>
            </g>
        </svg>
    }
}

#[component]
pub fn RunnerLoader(
    label: &'static str,
    #[prop(optional)] variant: LoaderVariant,
) -> impl IntoView {
    view! {
        <div class=variant.class() role="status" aria-live="polite">
            <div class="runner-loader__track">
                <RunnerFigure/>
                <span class="runner-loader__ground" aria-hidden="true"></span>
            </div>
            <span class="runner-loader__label">{label}</span>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::render;

    #[test]
    fn maps_variants() {
        assert_eq!(LoaderVariant::default(), LoaderVariant::Strip);
        assert!(LoaderVariant::Overlay.class().ends_with("--overlay"));
    }

    #[test]
    fn renders_runner_with_label() {
        let html = render(
            "/",
            || view! { <RunnerLoader label="Ladataan…" variant=LoaderVariant::Overlay/> },
        );
        assert!(html.contains("runner-loader--overlay"));
        assert!(html.contains("role=\"status\""));
        assert!(html.contains("Ladataan…"));
        assert!(html.contains("runner__leg--front"));
        assert!(html.contains("runner__headphones"));
        assert_eq!(html.matches("runner__shoe").count(), 2);
    }
}
