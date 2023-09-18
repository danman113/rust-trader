use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ItemType {
    pub key: String,
    pub name: String,
    pub value: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemIndex(usize);
pub struct ItemDatabase {
    items: Vec<ItemType>,
    item_map: HashMap<String, ItemIndex>,
}

impl ItemDatabase {
    pub fn insert(&mut self, item: ItemType) -> ItemIndex {
        let idx = ItemIndex(self.items.len());
        let key = item.key.clone();
        self.items.push(item);
        self.item_map.insert(key, idx);
        idx
    }

    pub fn get_index(&self, key: &str) -> Option<&ItemIndex> {
        self.item_map.get(key)
    }

    pub fn get_item_by_key(&self, key: &str) -> Option<&ItemType> {
        if let Some(idx) = self.item_map.get(key) {
            self.items.get(idx.0)
        } else {
            return None;
        }
    }

    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            item_map: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Inventory {
    pub items: HashMap<ItemIndex, u32>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
    pub fn add_item(&mut self, item: ItemIndex, amount: u32) -> u32 {
        let qty = self.items.entry(item).or_insert(0);
        *qty += amount;
        *qty
    }
}
