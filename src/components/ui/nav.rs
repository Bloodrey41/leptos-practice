use leptos::*;

#[component]
pub fn Nav(
    cx: Scope,
    children: Children
) -> impl IntoView {
    view! { cx,
    <div class="inline-flex h-10 items-center justify-center rounded-md bg-muted p-1 text-muted-foreground">
    {children(cx)}
    </div>
    }
}

#[component]
pub fn NavItem(
    cx: Scope,
    children: Children,
    href: &'static str
) -> impl IntoView {
    view! { cx,
    <a href=href class="inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50">
    {children(cx)}
    </a>
    }
}
