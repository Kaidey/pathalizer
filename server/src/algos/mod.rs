use crate::graph::{AdjacencyList, NodeCost, NodeId};

pub mod dijkstra;
pub mod astar;

enum AlgoError{
    InvalidStartPosition
}

pub struct PathFinderResult{
    pub ordered_path: Vec<NodeId>,
    pub total_cost: NodeCost
}

pub trait PathFinder{
    fn run(adj_list: &AdjacencyList, start: NodeId, end: NodeId) -> Option<PathFinderResult>;
}
