use leptos::*;

#[component]
pub fn Card(
    cx: Scope,
    children: Children,
    class: &'static str
) -> impl IntoView {
    view! { cx,
        <div
            class=format!("{} {}", "rounded-lg border bg-card text-card-foreground shadow-sm", class)
            >
        {children(cx)}
            </div>
    }
}
