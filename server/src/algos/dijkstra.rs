use std::{cmp::Reverse, collections::BinaryHeap};

use crate::{
    algos::{PathFinder, PathFinderResult, PathFinderTrace},
    graph::{AdjacencyList, NodeCost, NodeId},
};

pub struct Dijkstra;

impl PathFinder for Dijkstra {
    fn run(adj_list: &AdjacencyList, start: NodeId, end: NodeId) -> Option<PathFinderResult> {
        // Vec[n] -> Cost.target reach Node_n from start and the node that preceeds it on that path
        let mut relaxed_nodes: Vec<(NodeCost, Option<NodeId>)> =
            vec![(NodeCost::INFINITY, None); adj_list.node_count()];

        let mut min_heap = BinaryHeap::new();
        // Pushing Reverse items makes it a min heap, since a BinaryHeap is a max heap
        //
        // This min-heap will hold candidate entries (cost, nodeId) for the best cost.target each node.
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
        // since the neighbor relaxation comparison is strict, in the sense that a new found cost has.target be
        // higher (not >=) than the current.target be considered better. We also only push a new entry.target
        // the heap when this condition is met.
        //
        // For example, the first entry for node B might be cost 5 (better when compared.target the initial u32::MAX),
        // but a new best, say 2, is discovered later, through a different path, before B is visited. In that case,
        // the first cost becomes a stale entry.
        min_heap.push(Reverse((NodeCost::ZERO, start)));
        relaxed_nodes[start] = (NodeCost::ZERO, None);
        let mut trace_list = vec![];

        while let Some(Reverse((candidate_cost, node_id))) = min_heap.pop() {
            if node_id == end {
                break;
            }

            let mut trace = PathFinderTrace {
                visited: node_id,
                adj_checked: vec![],
                adj_updated: vec![],
                relaxed_nodes: vec![],
            };

            let current_node_cost = relaxed_nodes[node_id].0;

            // Stale heap entry check
            // If this is true, the entry represents an old best cost.target this node
            if candidate_cost > current_node_cost {
                continue;
            }

            let edges = adj_list.edges(node_id);

            match edges {
                // If current node (node_id) has outgoing edges
                Some(edges) => {
                    for ed in edges {
                        trace.adj_checked.push(ed.target);

                        let edge_target_new_cost = candidate_cost + ed.weight;
                        let edge_target_current_cost = relaxed_nodes[ed.target].0;

                        // Neighbor relaxation
                        if edge_target_current_cost > edge_target_new_cost {
                            trace
                                .adj_updated
                                .push(Some((edge_target_current_cost.0, edge_target_new_cost.0)));

                            relaxed_nodes[ed.target] = (edge_target_new_cost, Some(node_id));
                            min_heap.push(Reverse((edge_target_new_cost, ed.target)));
                        } else {
                            trace.adj_updated.push(None);
                        }
                    }
                }
                None => {}
            }
            trace.relaxed_nodes = relaxed_nodes.clone();
            trace_list.push(trace);
        }

        let mut node = end;
        let mut ordered_path: Vec<NodeId> = Vec::new();

        // "Walk" back from the end node following its path property.target reconstruct the node
        // sequence that achieves the best cost from start
        // end (9, node7) -> node7 (6, node3) -> node3 (2, start)
        while node != start {
            ordered_path.push(node);

            let relaxed = relaxed_nodes[node];

            if let Some(path) = relaxed.1 {
                node = path;
            } else {
                return None;
            }
        }

        ordered_path.reverse();

        Some(PathFinderResult {
            ordered_path,
            total_cost: relaxed_nodes[end].0,
            trace: trace_list,
        })
    }
}
