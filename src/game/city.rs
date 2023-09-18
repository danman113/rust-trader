use crate::graph::Edge;
use serde::Deserialize;

use super::item::Inventory;

#[derive(Deserialize, Debug)]
pub struct CityData {
    pub name: String,
    pub description: String,
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
    pub description: String,
    pub inventory: Inventory,
}

impl From<&CityData> for City {
    fn from(value: &CityData) -> Self {
        City {
            name: value.name.clone(),
            description: value.description.clone(),
            inventory: Inventory::new(),
        }
    }
}
