use serde::Deserialize;

use super::{city::CityData, item::ItemType};

#[derive(Deserialize, Debug)]
pub struct WorldData {
    pub starting_position: String,
    pub cities: Vec<CityData>,
    pub items: Vec<ItemType>,
}

pub fn deserialize_world(serialized_world: &str) -> Result<WorldData, toml::de::Error> {
    toml::from_str::<WorldData>(serialized_world)
}
