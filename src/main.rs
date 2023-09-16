use std::collections::HashMap;

use fastrand::Rng;
use rust_trader::game::city::{deserialize_world, CityData, RoadData, City};
use rust_trader::graph::{Edge, EdgeIndex, Graph, NodeIndex};

fn main() {
    let mut rng = Rng::with_seed(5);
    for _ in 0..=100 {
        let n = rng.i64(0..=10);
        println!("Hello, world! {n}");
    }

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
            let conn_a = cities.get(&city.name).expect(format!("Could not find city {} for road to {}", city.name, road.to).as_str());
            let conn_b = cities.get(&road.to).expect(format!("Could not find city {} for road to {}", city.name, road.to).as_str());
            world_map.insert_edge(road.clone(), *conn_a, *conn_b);
        }
    }

    println!(
        "{}",
        world
            .cities
            .iter()
            .map(|c| c.name.as_str())
            .collect::<Vec<&str>>()
            .join(", ")
    );

    println!(
        "{}",
        world_map.to_mermaid_format(|n, _| n.name.clone(), |e| format!("\"{} ({})\"", e.name.clone(), e.cost().to_string()))
    );
}
