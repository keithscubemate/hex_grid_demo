use std::fmt::Display;

fn main() {
    let width = 10;
    let height = 10;
    let mut hex_grid = HexGrid::new(width, height);

    set_print_clear(&mut hex_grid, 0, 0);
    set_print_clear(&mut hex_grid, 0, height - 1);
    set_print_clear(&mut hex_grid, width - 1, 0);
    set_print_clear(&mut hex_grid, width - 1, height - 1);

    set_print_clear(&mut hex_grid, 4, 5);
    set_print_clear(&mut hex_grid, 5, 4);
    set_print_clear(&mut hex_grid, 4, 4);
    set_print_clear(&mut hex_grid, 5, 5);
}

fn set_print_clear(hex_grid: &mut HexGrid, row: usize, col: usize) {
    hex_grid.select(row, col);
    hex_grid.highlight_neighbors(row, col);

    println!("{}", hex_grid);
    hex_grid.clear();
}

#[derive(Clone)]
enum CellState {
    None,
    Selected,
    Neighbor,
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "."),
            Self::Selected => write!(f, "S"),
            Self::Neighbor => write!(f, "N"),
        }
    }
}

struct HexGrid {
    width: usize,
    height: usize,
    cells: Vec<Vec<CellState>>,
}

impl HexGrid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![vec![CellState::None; width]; height],
        }
    }

    fn select(&mut self, row: usize, col: usize) {
        if row >= self.height || col >= self.width {
            // TODO(austin.jones): make this an error
            panic!()
        }
        let cell = &mut self.cells[row][col];
        *cell = CellState::Selected;
    }

    fn highlight_neighbors(&mut self, row: usize, col: usize) {
        if row >= self.height || col >= self.width {
            // TODO(austin.jones): make this an error
            panic!()
        }
        let directions: &[(isize, isize)] = &[
            // North
            (1, -1),
            (1, 0),
            // South
            (-1, 0),
            (-1, 1),
            // East
            (0, -1),
            // West
            (0, 1),
        ];

        for (r, c) in directions {
            let row = ((row as isize) + *r) as usize;
            let col = ((col as isize) + *c) as usize;

            if row >= self.height || col >= self.width {
                continue;
            }

            let cell = &mut self.cells[row][col];
            *cell = CellState::Neighbor;
        }
    }

    fn clear(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let cell = &mut self.cells[row][col];
                *cell = CellState::None;
            }
        }
    }
}

impl Display for HexGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for _ in 0..row {
                write!(f, " ")?;
            }
            for col in 0..self.width {
                let cell = &self.cells[row][col];
                write!(f, "{} ", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
