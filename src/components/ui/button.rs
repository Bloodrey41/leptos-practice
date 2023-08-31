use leptos::*;

#[component]
pub fn Button<F: Fn() -> bool + 'static>(
    cx: Scope,
    children: Children,
    class: &'static str,
    disabled: F
) -> impl IntoView {
    view! { cx,
    <button
        disabled=disabled
        class=format!("{} {}", "inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 h-10 px-4 py-2 bg-zinc-100 text-zinc-950 hover:bg-zinc-200 disabled:bg-zinc-400", class)
        >
        {children(cx)}
    </button>
    }
}
