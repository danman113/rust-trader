use std::collections::HashMap;

use dialoguer::{console::Term, Select};
use fastrand::Rng;
use rust_trader::game::city::{deserialize_world, City, CityData, RoadData};
use rust_trader::graph::{Edge, EdgeIndex, Graph, NodeIndex};

fn main() {
    let world =
        deserialize_world(include_str!("../data/world.toml")).expect("Could not read world file");
    // print!("{:?}", world);
    let mut world_map: Graph<City, RoadData> = Graph::new();
    let mut cities: HashMap<String, NodeIndex> = HashMap::new();
    for city in &world.cities {
        let idx = world_map.insert_node(city.into());
        cities.insert(city.name.clone(), idx);
    }

    for city in &world.cities {
        for road in &city.roads {
            let conn_a = cities.get(&city.name).expect(
                format!("Could not find city {} for road to {}", city.name, road.to).as_str(),
            );
            let conn_b = cities.get(&road.to).expect(
                format!("Could not find city {} for road to {}", city.name, road.to).as_str(),
            );
            world_map.insert_edge_undirected(road.clone(), *conn_a, *conn_b);
        }
    }

    let term = Term::stdout();

    let mut player_location = cities.get(&world.starting_position).unwrap();
    loop {
        term.clear_screen().unwrap();
        let current_city = world_map.get_node(*player_location).unwrap();
        let prompt = format!(
            "You are located at {}.\n{}\nWhat road would you like to take now?",
            current_city.name, current_city.description
        );
        let connections: Vec<_> = world_map
            .get_connections(*player_location)
            .unwrap()
            .iter()
            .collect();
        let options: Vec<String> = connections
            .iter()
            .map(|(edge, node)| {
                let edge = world_map.get_edge(*edge).unwrap();
                let node = world_map.get_node(*node).unwrap();
                format!("{} to {} ({} miles)", edge.name, node.name, edge.distance)
            })
            .collect();
        let choice = Select::new()
            .with_prompt(prompt)
            .items(&options)
            .default(0)
            .interact_on(&term)
            .expect("Chose invalid option");
        let (_, new_node) = connections
            .get(choice)
            .expect("Chose an invalid connection");
        player_location = new_node;
    }
}
