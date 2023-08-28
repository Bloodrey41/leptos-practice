use leptos::*;
use leptos_router::*;

use crate::components::counter::Counter;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
    <Router>
        <Routes>
        <Route path="/" view=Counter />
        </Routes>
        </Router>
    }
}
