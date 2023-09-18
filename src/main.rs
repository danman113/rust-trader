use dialoguer::Confirm;
use dialoguer::{console::Term, Select};
use fastrand::Rng;
use indicatif::{ProgressBar, ProgressDrawTarget};
use rust_trader::game::city::{City, RoadData};
use rust_trader::game::item::ItemDatabase;
use rust_trader::game::player::PlayerState;
use rust_trader::game::world::deserialize_world;
use rust_trader::graph::{Edge, EdgeIndex, Graph, NodeIndex};
use rust_trader::menu::{prompt_menu, MenuItem};
use std::collections::HashMap;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

struct GameState {
    world_map: Graph<City, RoadData>,
    item_database: ItemDatabase,
    term: Term,
    player: PlayerState,
    rng: Rng,
}

fn travel_to(state: &mut GameState, chosen_edge_idx: EdgeIndex, chosen_node_idx: NodeIndex) {
    let player_location = &state.player.position;
    let world_map = &state.world_map;
    let term = &state.term;
    let current_city = world_map.get_node(*player_location).unwrap();

    let chosen_edge = world_map.get_edge(chosen_edge_idx).unwrap();
    let chosen_node = world_map.get_node(chosen_node_idx).unwrap();

    let prompt = format!(
        "You are located at {}.\n{}\nWhat road would you like to take now?",
        current_city.name, current_city.description
    );

    let progress = ProgressBar::new(100);
    progress.set_draw_target(ProgressDrawTarget::term(term.clone(), 60));

    let miles_to_travel = chosen_edge.cost();
    let mut miles_traveled = 0;

    term.clear_screen().unwrap();
    term.write_line(&prompt).unwrap();
    term.write_line(
        format!(
            "Traveling on the {} to {}",
            chosen_edge.name, chosen_node.name
        )
        .as_str(),
    )
    .unwrap();

    for _ in 0..miles_to_travel {
        thread::sleep(Duration::from_millis(60));
        miles_traveled += 1;
        let event_odds = state.rng.u32(0..200);
        if event_odds == 0 {
            progress.suspend(|| {
                let a = Confirm::new()
                    .with_prompt("You see a homeless dude. Do you give him a nickel?")
                    .interact_on(&term)
                    .unwrap();
                if a {
                    state
                        .player
                        .inventory
                        .add_item(*state.item_database.get_index("berries").unwrap(), 3);
                    println!("{:?}", state.player.inventory);
                }
            })
        }

        progress.set_position(((miles_traveled as f32 / miles_to_travel as f32) * 100.0) as u64);
    }
    progress.finish();

    state.player.goto(chosen_node_idx);
}

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

    let mut item_database = ItemDatabase::new();
    for item in world.items {
        item_database.insert(item);
    }

    let term = Term::stdout();

    let player = PlayerState::new(*cities.get(&world.starting_position).unwrap());

    let rng = Rng::with_seed(30);

    let mut state = GameState {
        world_map,
        item_database,
        term,
        player,
        rng,
    };

    loop {
        let player_position = state.player.position;
        let current_city = state.world_map.get_node(player_position).unwrap();
        let prompt = format!(
            "You are located at {}.\n{}\nWhat road would you like to take now?",
            current_city.name, current_city.description
        );
        state.term.clear_screen().unwrap();
        let edge;
        let node;
        {
            let connections: Vec<_> = state
                .world_map
                .get_connections(state.player.position)
                .unwrap()
                .iter()
                .collect();
            let options: Vec<_> = connections
                .iter()
                .map(|(edge_idx, node_idx)| {
                    let edge = state.world_map.get_edge(*edge_idx).unwrap();
                    let node = state.world_map.get_node(*node_idx).unwrap();
                    format!("{} to {} ({} miles)", edge.name, node.name, edge.distance)
                })
                .collect();
            let choice = Select::new()
                .with_prompt(prompt)
                .items(&options)
                .default(0)
                .interact_on(&state.term)
                .expect("Chose invalid option");
            let (e, n) = connections.get(choice).unwrap();
            edge = *e;
            node = *n;
        }

        travel_to(&mut state, edge, node);
    }
}
