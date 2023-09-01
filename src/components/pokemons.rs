use leptos::*;
use serde::{Serialize, Deserialize};

use crate::components::ui::card::{Card, CardHeader, CardTitle, CardDescription, CardContent};
use crate::components::ui::badge::Badge;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct OfficialArtwork {
    front_default: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Other {
    #[serde(rename = "official-artwork")]
    official_artwork: OfficialArtwork
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Sprites {
    other: Other
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Type {
    name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Types {
    #[serde(rename = "type")]
    t: Type
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Pokemon {
    id: u8,
    name: String,
    sprites: Sprites,
    types: Vec<Types>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PokemonResult {
    name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PokemonsResponse {
    results: Vec<PokemonResult>
}

async fn get_pokemons() -> Result<PokemonsResponse, Box<dyn std::error::Error>> {
    let res = reqwest::get("https://pokeapi.co/api/v2/pokemon")
        .await?
        .json::<PokemonsResponse>()
        .await?;
    Ok(res)
}

#[component]
pub fn Pokemons(cx: Scope) -> impl IntoView {
    let pokemons = create_resource(cx,
        || (),
        |_| async move {
            let data = get_pokemons().await;
            let pokemons = data.unwrap();
            let results = pokemons.results;
            results
        }
    );

    view! { cx,
    <div class="absolute w-3/4 p-4 rounded top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-zinc-900">
        <Suspense
        fallback=move || view! { cx, <p>"Loading..."</p> }
    >
    {move || {
                 pokemons.read(cx).map(|pokemons| pokemons.into_iter()
                     .map(|pokemon| view! { cx, <PokemonCard pokemon_name=pokemon.name /> })
                     .collect_view(cx))
             }}
    </Suspense>
        </div>
    }
}

async fn get_pokemon(pokemon_name: String) -> Result<Pokemon, Box<dyn std::error::Error>> {
    let res = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_name))
        .await?
        .json::<Pokemon>()
        .await?;
    Ok(res)
}

#[component]
fn PokemonCard(
    cx: Scope,
    pokemon_name: String
) -> impl IntoView {
    let pokemon = create_resource(cx,
        || (),
        |_| async move {
            let data = get_pokemon(pokemon_name).await;
            let pokemon = data.unwrap();
            pokemon
        }
    );

    view! { cx,
    <Card class="inline-block w-[calc(33.3%-0.75rem)] mx-1">
    {pokemon.read(cx)
        .map(|pokemon| view! { cx,
            <CardHeader>
                //<img src=pokemon.sprites.other.official_artwork />
                <CardDescription>
                "#0001"
                </CardDescription>
                <CardTitle>
                {pokemon.name}
                </CardTitle>
                </CardHeader>
                <CardContent>
                <Badge>"grass"</Badge>
                </CardContent>
        })}
        </Card>
    }
}
