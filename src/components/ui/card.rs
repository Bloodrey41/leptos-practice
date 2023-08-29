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

#[component]
pub fn CardHeader(
    cx: Scope,
    children: Children,
    class: &'static str
) -> impl IntoView {
    view! { cx,
    <div
        class=format!("{} {}", "flex flex-col space-y-1.5 p-6", class)
        >
        {children(cx)}
    </div>
    }
}

#[component]
pub fn CardTitle(
    cx: Scope,
    children: Children,
    class: &'static str
) -> impl IntoView {
    view! { cx,
    <div
        class=format!("{} {}", "text-2xl font-semibold leading-none tracking-tight", class)
        >
        {children(cx)}
    </div>
    }
}

#[component]
pub fn CardDescription(
    cx: Scope,
    children: Children,
    class: &'static str
) -> impl IntoView {
    view! { cx,
    <div
        class=format!("{} {}", "text-sm text-muted-foreground", class)
        >
        {children(cx)}
    </div>
    }
}

#[component]
pub fn CardContent(
    cx: Scope,
    children: Children,
    class: &'static str
) -> impl IntoView {
    view! { cx,
    <div
        class=format!("{} {}", "p-6 pt-0", class)
        >
        {children(cx)}
    </div>
    }
}

#[component]
pub fn CardFooter(
    cx: Scope,
    children: Children,
    class: &'static str
) -> impl IntoView {
    view! { cx,
    <div
        class=format!("{} {}", "flex items-center p-6 pt-0", class)
        >
        {children(cx)}
    </div>
    }
}
