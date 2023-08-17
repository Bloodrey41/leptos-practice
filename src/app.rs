use leptos::{*, leptos_dom::console_log};

const INITIAL_COUNT: u8 = 0;

const MAX_COUNT: u8 = 10;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, INITIAL_COUNT);

    create_effect(cx, move |_| {
        let count = format!("count: {}", count.get());

        console_log(&count);
    });

    let handle_increase_click = move |_| {
        set_count.update(|n| *n += 1);
    };

    let handle_decrease_click = move |_| {  
        set_count.update(|n| *n -= 1);
    };

    view! { cx,
    <div>
        <button
        on:click=handle_decrease_click
        disabled=move || count.get() == INITIAL_COUNT
        >
        "Decrease"
        </button>
        <span
        class:even=move || count.get() % 2 == 0
              class:odd=move || count.get() % 2 == 1
                    >
                    " "{count}" "
                    </span>
                    <button
                    on:click=handle_increase_click
                    disabled=move || count.get() == MAX_COUNT
                    >
                    "Increase"
                    </button>
                    <br />
                    <progress
                    max=MAX_COUNT
                    value=move || count.get()
                    />
                    </div>
    }
}
