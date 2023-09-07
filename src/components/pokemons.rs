use leptos::*;

use crate::components::ui::card::{Card, CardHeader, CardTitle, CardDescription, CardContent};
use crate::components::ui::badge::Badge;
use crate::components::ui::button::Button;
use crate::model::pokemon::PokemonMac;

#[component]
pub fn Pokemons(cx: Scope) -> impl IntoView {
    // (limit = 6, offset = 0)
    let (page, set_page) = create_signal(cx, (6, 0));

    let (pokemons, set_pokemons) = create_signal(cx, Vec::new());

    let results = create_resource(cx,
        move || page.get(),
        |page| async move {
            let data = PokemonMac::list(page.0, page.1).await;
            let pokemons = data.unwrap();
            let results = pokemons.results;
            results
        }
    );

    create_effect(cx, move |_| {
        results.read(cx).map(|results| results.into_iter().for_each(|result| {
            set_pokemons.update(|pokemons| pokemons.push(result.name));
        }));
    });

    let handle_show_more_click = move |_| {
        set_page.update(|page| *page = (3, page.1 + if page.1 == 0 { 6 } else { 3 }));
    };

    view! { cx,
    <div class="absolute w-3/4 p-4 rounded top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-zinc-900 h-5/6 overflow-scroll">
        <For
        each=move || pokemons.get()
        key=|pokemon| pokemon.clone()
        view=move |cx, pokemon| view! { cx, <PokemonCard pokemon_name=pokemon /> }
    />
        <Button
        on:click=handle_show_more_click
        class="inline-block w-full text-2xl text-center"
        disabled=|| false
        >
        "Show more"
        </Button>
        </div>
    }
}

#[component]
fn PokemonCard(
    cx: Scope,
    pokemon_name: String
) -> impl IntoView {
    let pokemon = create_resource(cx,
        move || pokemon_name.clone(),
        |name| async move {
            let data = PokemonMac::get(name).await;
            let pokemon = data.unwrap();
            pokemon
        }
    );

    view! { cx,
    <Card class="inline-block w-[calc(33.3%-0.75rem)] m-1">
        <Suspense
        fallback=move || view! { cx, <p>"Loading..."</p> }
    >
    {move || pokemon.read(cx)
        .map(|pokemon| view! { cx,
            <CardHeader>
                <img src=pokemon.sprites.other.official_artwork.front_default />
                <CardDescription>
                #{pokemon.id}
                </CardDescription>
                <CardTitle>
                {pokemon.name}
            </CardTitle>
                </CardHeader>
                <CardContent>
                {pokemon.types.into_iter()
                    .map(|typo| view! { cx, <Badge class="mx-px">{typo.typo.name}</Badge> })
                        .collect_view(cx)}
            </CardContent>
        })}
        </Suspense>
            </Card>
    }
}
