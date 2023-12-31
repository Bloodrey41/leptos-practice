use leptos::*;

use app::App;

mod app;
mod components;
mod model;

fn main() {
    mount_to_body(|cx| view!{ cx, <App /> })
}
