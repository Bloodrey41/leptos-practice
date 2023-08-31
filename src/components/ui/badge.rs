use leptos::*;

#[component]
pub fn Badge(
    cx: Scope,
    children: Children,
    #[prop(optional)]
    class: &'static str
) -> impl IntoView {
    view! { cx,
    <div class=format!("{} {}", "inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 border-transparent bg-zinc-100 text-zinc-950 hover:bg-zinc-200", class)>
    {children(cx)}
    </div>
    }
}
