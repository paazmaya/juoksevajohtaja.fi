//! Juokseva Johtaja podcast site: a Leptos single-page application with a persistent audio player.

// Leptos' statically typed views nest deeply.
#![recursion_limit = "512"]

pub mod app;
pub mod components;
pub mod data;
pub mod pages;
pub mod player;
pub mod web;

#[cfg(feature = "ssr")]
pub mod render;

#[cfg(test)]
pub(crate) mod test_support;

pub use app::App;
