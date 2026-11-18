use std::{cmp::Reverse, collections::BinaryHeap};

use crate::{
    algos::{PathFinder, PathFinderResult},
    graph::{AdjacencyList, NodeId},
};

pub struct Dijkstra;

impl PathFinder for Dijkstra {
    fn run(adj_list: &AdjacencyList, start: NodeId, end: NodeId) -> Option<PathFinderResult> {
        // Each item is (cost, path), while the id of the referenced node is its position on the Vec
        // This will hold the best cost to reach each node from start and what node precedes them in
        // that best path
        let mut relaxed_nodes: Vec<(u32, Option<NodeId>)> =
            vec![(u32::MAX, None); adj_list.node_count()];

        let mut min_heap = BinaryHeap::new();
        // Pushing Reverse items makes it a min heap, since a BinaryHeap is a max heap
        //
        // This min-heap will hold candidate entries (cost, nodeId) for the best cost to each node.
        // There is, at all times, at least one entry per discovered but unvisited node, containing
        // the best found cost so far. If multiple exist, only one is the best, while the others are
        // considered stale.
        //
        // A node is considered visited when it's non-stale entry is popped, at which point its cost
        // is final and recorded in the corresponding index on the relaxed_nodes Vec.
        //
        // Stale entries are discarded by the cost > current_cost check after their corresponding
        // node is visited.
        //
        // There is no possibility for a tie over best cost (resulting in multiple non-stale entries)
        // since the neighbor relaxation comparison is strict, in the sense that a new found cost has to be
        // higher (not >=) than the current to be considered better. We also only push a new entry to
        // the heap when this condition is met.
        //
        // For example, the first entry for node B might be cost 5 (better when compared to the initial u32::MAX),
        // but a new best, say 2, is discovered later, through a different path, before B is visited. In that case,
        // the first cost becomes a stale entry.
        min_heap.push(Reverse((0u32, start)));
        relaxed_nodes[start.id()] = (0, None);

        while let Some(Reverse((cost, id))) = min_heap.pop() {
            if id == end {
                break;
            }

            let current_cost = relaxed_nodes[id.id()].0;

            // Stale heap entry check
            // If this is true, the entry represents an old best cost to this node
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

                        // Neighbor relaxation
                        if current_n_cost > new_n_cost {
                            relaxed_nodes[n.to.id()] = (new_n_cost, Some(id));
                            min_heap.push(Reverse((new_n_cost, n.to)));
                        }
                    }
                }
                None => {}
            }
        }

        let mut node = end;
        let mut ordered_path: Vec<NodeId> = Vec::new();

        // "Walk" back from the end node following its path property to reconstruct the node
        // sequence that achieves the best cost from start
        // end (9, node7) -> node7 (6, node3) -> node3 (2, start)
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
