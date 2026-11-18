use crate::algos::{PathFinder, PathFinderResult};

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

impl NodeId {
    pub fn id(&self) -> usize {
        self.0
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Edge {
    pub to: NodeId,
    pub cost: u32,
}

pub struct AdjacencyList(Vec<Vec<Edge>>);

impl AdjacencyList {
    pub fn calc_sp<PF: PathFinder>(&self, start: NodeId, end: NodeId) -> Option<PathFinderResult> {
        PF::run(self, start, end)
    }

    pub fn neighbors(&self, node: NodeId) -> Option<&Vec<Edge>> {
        self.0.get(node.id())
    }

    pub fn node_count(&self) -> usize {
        self.0.len()
    }

    // TODO: Remove after testing
    pub fn from_grid(grid: &[Vec<u32>]) -> Self {
        let rows = grid.len();
        let cols = grid.first().map_or(0, |row| row.len());
        let id = |r: usize, c: usize| NodeId(r * cols + c);

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
                        if id(r, c).0 == 3 || id(r, c).0 == 8 || id(r, c).0 == 9 {
                            continue;
                        }
                        if id(nr, nc).0 == 3 || id(nr, nc).0 == 8 || id(nr, nc).0 == 9 {
                            continue;
                        }
                        adj[id(r, c).0].push(Edge {
                            to: id(nr, nc),
                            cost: grid[nr][nc],
                        });
                    }
                }
            }
        }
        for (id, edges) in adj.iter().enumerate() {
            let list: Vec<(usize, u32)> = edges.iter().map(|e| (e.to.0, e.cost)).collect();
            println!("{id} -> {list:?}");
        }

        Self(adj)
    }
}
