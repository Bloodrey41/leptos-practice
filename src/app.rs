use leptos::*;
use leptos_router::*;

use crate::components::{counter::Counter, ui::nav::{Nav, NavItem}};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
    <Router>
        <main>
        <Nav>
        <NavItem href="/counter">
        "Counter"
        </NavItem>
        <NavItem href="/pokemons">
        "Pokemon list"
        </NavItem>
        <Routes>
        <Route path="/counter" view=Counter />
        </Routes>
        </Nav>
        </main>
        </Router>
    }
}
