use leptos::*;

use app::App;

mod app;

fn main() {
    mount_to_body(|cx| view!{ cx, <App /> })
}
