use leptos::mount::mount_to_body;

mod app;
mod components;
mod pages;
mod api;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}