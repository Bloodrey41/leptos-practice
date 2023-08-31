use std::collections::HashMap;

use leptos::*;

use crate::components::ui::card::{Card, CardHeader, CardTitle, CardDescription, CardContent};
use crate::components::ui::badge::Badge;

async fn get_pokemons() -> HashMap<String, String> {
    let res = reqwest::get("https://pokeapi.co/api/v2/pokemon/ditto")
        .await.unwrap()
        .json::<HashMap<String, String>>()
        .await.unwrap();
    res
}

#[component]
pub fn Pokemons(cx: Scope) -> impl IntoView {
    create_resource(cx,
        || (),
        |_| async move {
            let data = get_pokemons().await;
            log!("{:#?}", data);
        }
        );

    view! { cx,
    <div class="absolute w-3/4 p-4 rounded top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-zinc-900">
        <Card class="inline-block w-[calc(33.3%-0.75rem)] mx-1">
        <CardHeader>
        <img src="https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/other/official-artwork/1.png" />
        <CardDescription>
        "#0001"
        </CardDescription>
        <CardTitle>
        "Bulbasaur"
        </CardTitle>
        </CardHeader>
        <CardContent>
        <Badge>"grass"</Badge>
        </CardContent>
        </Card>
        <Card class="inline-block w-[calc(33.3%-0.75rem)] mx-1">
        <CardHeader>
        <img src="https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/other/official-artwork/1.png" />
        <CardDescription>
        "#0001"
        </CardDescription>
        <CardTitle>
        "Bulbasaur"
        </CardTitle>
        </CardHeader>
        <CardContent>
        <Badge>"grass"</Badge>
        </CardContent>
        </Card>
        <Card class="inline-block w-[calc(33.3%-0.75rem)] mx-1">
        <CardHeader>
        <img src="https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/other/official-artwork/1.png" />
        <CardDescription>
        "#0001"
        </CardDescription>
        <CardTitle>
        "Bulbasaur"
        </CardTitle>
        </CardHeader>
        <CardContent>
        <Badge>"grass"</Badge>
        </CardContent>
        </Card>
        </div>
    }
}
