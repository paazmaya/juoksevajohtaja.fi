//! Routed pages. Each page sets its own `<head>` through [`crate::components::PageHead`].

pub mod episode;
pub mod game;
pub mod home;
pub mod not_found;

pub use episode::{EpisodePage, EpisodeRoute};
pub use game::{GamePage, GAME_APP_PATH};
pub use home::HomePage;
pub use not_found::NotFoundPage;
