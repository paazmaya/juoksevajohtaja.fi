//! Browser entry point (built by Trunk). Replaces the pre-rendered markup with the live application.

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        use juoksevajohtaja_site::{web, App};

        console_error_panic_hook::set_once();
        let root = web::prepare_document();
        leptos::mount::mount_to(root, App).forget();
        web::finish_boot();
    }
}
