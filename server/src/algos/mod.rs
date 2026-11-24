use crate::graph::{AdjacencyList, NodeCost, NodeId};

pub mod dijkstra;
pub mod astar;

pub struct PathFinderTrace{
    pub visited: NodeId,
    pub adj_checked: Vec<NodeId>,
    // adj_updated will mirror adj_checked having an (old_cost, new_cost) tuple if the node was
    // updated. E.g adj_checked = [12,5,10] -> adj_updated = [Some((6,2)), None, Some((5,3))]
    // The tuple will be mapped to {old_cost: x, new_cost: y} or null if None as JSON
    pub adj_updated: Vec<Option<(u32,u32)>>,
    pub relaxed_nodes: Vec<(NodeCost, Option<NodeId>)>
}

pub struct PathFinderResult{
    pub ordered_path: Vec<NodeId>,
    pub total_cost: NodeCost,
    pub trace: Vec<PathFinderTrace>
}

pub trait PathFinder{
    fn run(adj_list: &AdjacencyList, start: NodeId, end: NodeId) -> Option<PathFinderResult>;
}
