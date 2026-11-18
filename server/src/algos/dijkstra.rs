use std::{cmp::Reverse, collections::BinaryHeap};

use crate::{
    algos::{PathFinder, PathFinderResult},
    graph::{AdjacencyList, NodeId},
};

pub struct Dijkstra;

impl PathFinder for Dijkstra {
    fn run(adj_list: &AdjacencyList, start: NodeId, end: NodeId) -> Option<PathFinderResult> {
        let mut min_heap = BinaryHeap::new();
        // Each item is (cost, path), while the id of the node is its position on the Vec
        // This will hold the least cost path to start for each node
        let mut relaxed_nodes: Vec<(u32, Option<NodeId>)> =
            vec![(u32::MAX, None); adj_list.node_count()];

        // (cost, NodeId)
        // Pushing Reverse items makes it a min heap, since a BinaryHeap is a max heap
        // This will hold all the discovered but not visited nodes, keeping the least cost one at
        // the top
        min_heap.push(Reverse((0u32, start)));
        relaxed_nodes[start.id()] = (0, None);

        while let Some(Reverse((cost, id))) = min_heap.pop() {
            if id == end {
                break;
            }

            let current_cost = relaxed_nodes[id.id()].0;

            if cost > current_cost {
                continue;
            }

            let neighs = adj_list.neighbors(id);

            match neighs {
                // If current node (id) has neighbors
                Some(neigh_list) => {
                    for n in neigh_list {
                        let new_n_cost = cost + n.cost;
                        let current_n_cost = relaxed_nodes[n.to.id()].0;

                        if current_n_cost > new_n_cost {
                            relaxed_nodes[n.to.id()] = (new_n_cost, Some(id));
                        }

                        min_heap.push(Reverse((new_n_cost, n.to)));
                    }
                }
                None => {}
            }
        }

        let mut node = end;
        let mut ordered_path: Vec<NodeId> = Vec::new();

        while node != start {
            ordered_path.push(node);

            let relaxed = relaxed_nodes[node.id()];

            if let Some(path) = relaxed.1 {
                node = path;
            } else {
                return None;
            }
        }

        ordered_path.reverse();

        Some(PathFinderResult {
            ordered_path,
            total_cost: relaxed_nodes[end.id()].0,
        })
    }
}
