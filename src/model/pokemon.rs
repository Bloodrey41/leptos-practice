use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pokemon {
    pub id: u8,
    pub name: String,
    pub sprites: Sprites,
    pub types: Vec<Types>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sprites {
    pub other: Other
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Other {
    #[serde(rename = "official-artwork")]
    pub official_artwork: OfficialArtwork
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OfficialArtwork {
    pub front_default: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Types {
    #[serde(rename = "type")]
    pub typo: Type
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Type {
    pub name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PokemonsResponse {
    pub results: Vec<PokemonResult>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PokemonResult {
    pub name: String
}

pub struct PokemonMac;

impl PokemonMac {
    pub async fn list(limit: u8, offset: u8) -> Result<PokemonsResponse, Box<dyn std::error::Error>> {
        let res = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon?limit={}&offset={}", limit, offset))
            .await?
            .json::<PokemonsResponse>()
            .await?;
        Ok(res)
    }

    pub async fn get(pokemon_name: String) -> Result<Pokemon, Box<dyn std::error::Error>> {
        let res = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_name))
            .await?
            .json::<Pokemon>()
            .await?;
        Ok(res)
    }
}
