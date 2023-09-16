use serde::Deserialize;
use crate::graph::Edge;

#[derive(Deserialize, Debug)]
pub struct CityData {
    pub name: String,
    pub roads: Vec<RoadData>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RoadData {
    pub name: String,
    pub to: String,
    pub distance: u32,
}

impl Edge for RoadData {
	fn cost(&self) -> u32 {
		self.distance
	}
}

pub struct City {
	pub name: String,
}

impl From<&CityData> for City {
	fn from(value: &CityData) -> Self {
		City {
			name: value.name.clone()
		}
	}
}

#[derive(Deserialize, Debug)]
pub struct WorldData {
    pub starting_position: String,
    pub cities: Vec<CityData>,
}

pub fn deserialize_world(serialized_world: &str) -> Result<WorldData, toml::de::Error> {
    toml::from_str::<WorldData>(serialized_world)
}
