use std::ops::Add;

use crate::algos::{PathFinder, PathFinderResult};

const MAX_NODES: usize = 1000;
// 8 edges per node
const MAX_EDGES: usize = 8000;

#[derive(Debug)]
pub enum ValidationError {
    Empty,
    TooManyNodes,
    TooManyEdges,
    EdgeOutOfBounds { from: usize, to: usize },
    NodeOutOfBounds(NodeId),
}

pub type NodeId = usize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeCost(pub u32);

impl NodeCost {
    pub const ZERO: Self = Self(0);
    pub const INFINITY: Self = Self(u32::MAX);
}

impl Add<u32> for NodeCost {
    type Output = Self;
    fn add(self, rhs: u32) -> Self::Output {
        Self(self.0 + rhs)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Edge {
    pub target: NodeId,
    pub weight: u32,
}

pub struct AdjacencyList(Vec<Vec<Edge>>);

impl AdjacencyList {
    pub fn new(adj: Vec<Vec<Edge>>) -> Result<Self, ValidationError> {
        if adj.is_empty() {
            return Err(ValidationError::Empty);
        }
        if adj.len() > MAX_NODES {
            return Err(ValidationError::TooManyNodes);
        }
        let mut edge_count = 0usize;
        for (curr_node, edges) in adj.iter().enumerate() {
            edge_count += edges.len();
            if edge_count > MAX_EDGES {
                return Err(ValidationError::TooManyEdges);
            }
            // Each entry in the main vec corresponds to a graph node and the value in that entry
            // its edges. So, the length of the main vec is the total number of nodes in the graph.
            // Thus, if any edge points to a node which index is greater than that, it is OOB
            if let Some(e) = edges.iter().find(|e| e.target >= adj.len()) {
                return Err(ValidationError::EdgeOutOfBounds {
                    from: curr_node,
                    to: e.target,
                });
            }
        }
        Ok(Self(adj))
    }

    pub fn calc_sp<PF: PathFinder>(&self, start: NodeId, end: NodeId) -> Option<PathFinderResult> {
        PF::run(self, start, end)
    }

    pub fn edges(&self, node: NodeId) -> Option<&Vec<Edge>> {
        self.0.get(node)
    }

    pub fn node_count(&self) -> usize {
        self.0.len()
    }

    // TODO: Remove after testing
    pub fn from_grid(grid: &[Vec<u32>]) -> Self {
        let rows = grid.len();
        let cols = grid.first().map_or(0, |row| row.len());

        let mut adj: Vec<Vec<Edge>> = vec![Vec::new(); rows * cols];

        for r in 0..rows {
            for c in 0..cols {
                let candidates = [
                    r.checked_sub(1).map(|r| (r, c)), // up
                    Some((r + 1, c)),                 // down
                    c.checked_sub(1).map(|c| (r, c)), // left
                    Some((r, c + 1)),                 // right
                ];

                for (nr, nc) in candidates.into_iter().flatten() {
                    if nr < rows && nc < cols {
                        let curr_id = r * cols + c;
                        let n_id = nr * cols + nc;
                        adj[curr_id].push(Edge {
                            target: n_id,
                            weight: grid[nr][nc],
                        });
                    }
                }
            }
        }
        for (id, edges) in adj.iter().enumerate() {
            let list: Vec<(NodeId, u32)> = edges.iter().map(|e| (e.target, e.weight)).collect();
            println!("{id} -> {list:?}");
        }

        Self(adj)
    }
}
