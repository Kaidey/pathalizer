use std::{collections::HashMap, fmt};

pub enum GridError {
    InvalidDimensions,
    InvalidStartPosition,
    CellHasMultipleWeights,
    StartIsWall,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Cell {
    // The distance from this cell to the start cell
    dist: u8,
    // The weight of moving to this cell
    // In a grid, since there are no edges, the weight is applied when travelling to a cell from any
    // of its direct neighbors
    weight: u8,
    path: Option<(u8, u8)>,
    is_wall: bool,
    is_visited: bool,
}

impl Cell {
    pub fn new(dist: u8, weight: u8, is_wall: bool) -> Self {
        Self {
            dist: dist,
            weight: weight,
            path: None,
            is_wall,
            is_visited: false,
        }
    }

    pub fn is_wall(&self) -> bool {
        self.is_wall
    }

    pub fn is_visited(&self) -> bool {
        self.is_visited
    }

    pub fn dist(&self) -> u8 {
        self.dist
    }

    pub fn weight(&self) -> u8 {
        self.weight
    }

    pub fn update_dist(&mut self, new_dist: u8, path: (u8, u8)) {
        self.dist = new_dist + self.weight;
        self.path = Some(path);
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:03},{},{:6}", self.dist, self.weight, self.is_wall)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid(Vec<Vec<Cell>>);

impl Grid {
    // Dims is (cols, rows)
    // Start is (col, row)
    // Walls is (col, row)
    // Weighted is (col, row, weight)
    pub fn new(
        dims: (u8, u8),
        start: (u8, u8),
        walls: Vec<(u8, u8)>,
        weighted: Vec<(u8, u8, u8)>,
    ) -> Result<Self, GridError> {
        let (cols, rows) = dims;

        if cols <= 0 || rows <= 0 {
            return Err(GridError::InvalidDimensions);
        }

        if start.0 < 0 || start.0 >= cols || start.1 < 0 || start.1 >= rows {
            return Err(GridError::InvalidStartPosition);
        }

        let mut weights: HashMap<(u8, u8), u8> = HashMap::new();

        for w in weighted {
            let (wc, wr, w) = w;

            if weights.contains_key(&(wc, wr)) {
                return Err(GridError::CellHasMultipleWeights);
            }

            weights.insert((wc, wr), w);
        }

        let mut grid: Vec<Vec<Cell>> = Vec::with_capacity(cols as usize);

        // Init grid
        for c in 0..cols {
            let mut col: Vec<Cell> = Vec::new();
            for r in 0..rows {
                let mut dist = u8::MAX;
                let mut weight = 1;
                let mut is_wall = false;

                // TODO: This if else hierarchy ignores invalid input like weighted walls or the 
                // start cell marked as a wall. Might be worth revisiting if the grid creation 
                // should fail in such cases so the client knows it is passing invalid input
                if (c, r) == start {
                    dist = 0;
                } else if walls.contains(&(c, r)) {
                    is_wall = true;
                } else if let Some(w) = weights.remove(&(c, r)) {
                    weight = w;
                }

                col.push(Cell::new(dist, weight, is_wall));
            }
            grid.push(col);
        }

        Ok(Self(grid))
    }

    pub fn cell_from_coords(&self, col: usize, row: usize) -> Option<&Cell> {
        let cell_col = self.0.get(col);

        // Out of bounds checks
        match cell_col {
            // If not out of bounds, returns the array at position col
            Some(col) => {
                let cell = col.get(row);

                match cell {
                    // If not out of bounds, returns the cell at position row on the array at col
                    Some(c) => return Some(c),
                    None => return None,
                }
            }
            None => return None,
        }
    }

    pub fn neighbours(&self, col: usize, row: usize) -> impl Iterator<Item = (u8, u8)> {
        let candidates = [
            (col + 1, row),
            (col - 1, row),
            (col, row + 1),
            (col, row - 1),
        ];

        // Returns iterator over only the candidates that pass both checks
        candidates.into_iter().filter_map(|(c, r)| {
            // Out of bounds check
            let cell = self.0.get(c)?.get(r)?;
            // Is wall check
            (!cell.is_wall() && !cell.is_visited()).then_some((c as u8, r as u8))
        })
    }

    pub fn relax(&mut self, col: usize, row: usize) {}
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let owned = self.0.to_owned();

        // Get capacity of first column. All have the same
        let cells_per_col = owned[0].capacity();

        // Traverse elements at pos i for every col
        for i in 0..cells_per_col {
            for col in owned.clone() {
                write!(f, "{}  ", col[i])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
