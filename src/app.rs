use leptos::*;

use crate::components::counter::Counter;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
    <Counter />
    }
}
