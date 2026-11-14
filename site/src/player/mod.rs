//! Site-wide podcast player state. One instance is provided at the application root and survives navigation.

mod state;

pub use state::{
    parse_snapshot, AudioBackend, PlayerState, SKIP_BACK_SECONDS, SKIP_FORWARD_SECONDS,
};
