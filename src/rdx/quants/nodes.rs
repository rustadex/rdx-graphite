
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::tag::Tag;
use super::NodeID;

/// Represents a primary entity in the graph.
// Renamed from GraphNode to Node
#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub id: NodeID,
    pub tags: HashMap<String, Tag>,
    pub edges: HashMap<String, NodeID>,
}

// Renamed from GraphNode to Node
impl Node {
    /// Creates a new, empty Node with the given ID.
    pub fn new(id: NodeID) -> Self {
        Node { // Renamed here
            id,
            tags: HashMap::new(),
            edges: HashMap::new(),
        }
    }
}
