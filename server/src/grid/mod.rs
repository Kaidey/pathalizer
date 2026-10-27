use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Cell {
    // The distance from this cell to the start cell
    dist: u8,
    // The cost of moving to this cell
    cost: u8,
    is_wall: bool,
}

impl Cell {
    pub fn new(cost: Option<u8>, is_wall: bool) -> Self {
        Self {
            dist: u8::MAX,
            cost: cost.unwrap_or(1),
            is_wall: is_wall,
        }
    }

    pub fn is_wall(&self) -> bool {
        self.is_wall
    }

    pub fn update_dist(&mut self, accumulated_dist: u8) {
        self.dist = accumulated_dist + self.cost
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:03},{},{:6}", self.dist, self.cost, self.is_wall)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid(Vec<Vec<Cell>>);

impl Grid {
    // Dims is (rows, cols)
    // Walls is (x_coord, y_coord)
    // Weighted is (x_coord, y_coord, weight)
    pub fn new(
        dims: (u8, u8),
        start: (u8, u8),
        walls: Vec<(u8, u8)>,
        weighted: Vec<(u8, u8, u8)>,
    ) -> Self {
        let (rows, cols) = dims;

        let mut grid: Vec<Vec<Cell>> = Vec::with_capacity(cols as usize);
        let start_dist = u8::MAX;
        let base_weight = 1;

        let base_cell = Cell {
            dist: start_dist,
            cost: base_weight,
            is_wall: false,
        };

        // Init grid columns
        for _ in 0..cols {
            let col: Vec<Cell> = vec![base_cell; rows as usize];
            grid.push(col);
        }

        // Set wall (non-traversible) cells
        for wall in walls {
            let (x, y) = wall;

            grid[x as usize][y as usize].is_wall = true;
        }

        // Set weighted cells. Weight is analogous to the edge weight of a graph
        for wei_c in weighted {
            let (x, y, weight) = wei_c;

            grid[x as usize][y as usize].cost = weight;
        }

        let (start_x, start_y) = start;

        // Set start cell distance to 0
        grid[start_x as usize][start_y as usize].dist = 0;

        Self(grid)
    }
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
