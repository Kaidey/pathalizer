use crate::{
    algos::dijkstra::Dijkstra,
    graph::{AdjacencyList, Edge, NodeId},
};

pub mod algos;
pub mod api;
pub mod graph;

fn main() {
    let grid = vec![
        vec![1, 1, 1, 4, 10],
        vec![1, 1, 10, 10, 10],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 7, 1],
        vec![1, 1, 1, 1, 8],
    ];

    let adj = AdjacencyList::from_grid(&grid);


    // TODO: Check for out of bounds IDs
    // TODO: Check start/end are walls (no adjacency in adj). This will be done on FE as well
    //          - Using the same wall as start and end returns no path and cost 0 as it should if
    //          the node was not a wall
    //          - Using valid start and wall end returns no path.
    //          - Using wall start and valid end returns no path
    //          - Using wall start and wall end returns no path
    //          Should error if end or start are
    //          walls?
    let res = adj.calc_sp::<Dijkstra>(NodeId(1), NodeId(22));

    match res {
        Some(r) => {
            println!("Path: {:?}", r.ordered_path);
            println!("Total cost: {}", r.total_cost);
        }
        None => {
            println!("No path")
        }
    }
}
