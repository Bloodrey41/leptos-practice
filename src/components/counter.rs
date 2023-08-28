use leptos::*;

use crate::components::ui::button::Button;

const INITIAL_COUNT: u8 = 0;

const MAX_COUNT: u8 = 10;

#[component]
pub fn Counter(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, INITIAL_COUNT);

    create_effect(cx, move |_| {
        let count = format!("count: {}", count.get());
        log!("{}", count);
    });

    let handle_increase_click = move |_| {
        set_count.update(|n| *n += 1);
    };

    let handle_decrease_click = move |_| {  
        set_count.update(|n| *n -= 1);
    };

    view! { cx,
    <div class="absolute w-1/4 p-4 rounded top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-zinc-900">
        <Button
        on:click=handle_decrease_click
        disabled=move || count.get() == INITIAL_COUNT
        class="w-1/3"
        >
        "Decrease"
        </Button>
        <span
        class="inline-block w-1/3 text-xl text-center"
        class:even=move || count.get() % 2 == 0
              class:odd=move || count.get() % 2 == 1
                    >
                    " "{count}" "
                    </span>
                    <Button
                    on:click=handle_increase_click
                    disabled=move || count.get() == MAX_COUNT
                    class="w-1/3"
                    >
                    "Increase"
                    </Button>
                    <br />
                    <progress
                    max=MAX_COUNT
                    value=move || count.get()
                    class="relative w-full h-4 mt-2 overflow-hidden rounded-full bg-zinc-100"
                    />
                    <br />
                    <span
                    class="inline-block w-full text-2xl text-center"
                    >
                    <Show
                    when=move || count.get() % 2 == 0
                    fallback=|cx| view! { cx, "Odd" }
    >
        "Even"
        </Show>
        </span>
        </div>
    }
}
