use std::collections::HashSet;

use crate::graph::NodeIndex;

use super::item::Inventory;

pub struct PlayerState {
    pub position: NodeIndex,
    pub inventory: Inventory,
    pub visited_places: HashSet<NodeIndex>,
}

impl PlayerState {
    pub fn new(position: NodeIndex) -> Self {
        Self {
            position,
            inventory: Inventory::new(),
            visited_places: HashSet::new(),
        }
    }

    pub fn goto(&mut self, node: NodeIndex) {
        self.position = node;
        self.visited_places.insert(node);
    }
}
