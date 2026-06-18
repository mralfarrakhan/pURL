use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Simple {}

#[derive(Debug, Deserialize, Serialize)]
pub enum Item {
    Simple(Simple),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Collection {
    items: BTreeMap<String, Item>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Collections {
    collections: BTreeMap<String, Collection>,
}
