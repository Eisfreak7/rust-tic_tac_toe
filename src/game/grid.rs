use super::{CellState, PlayerId};

pub struct Grid {
    // inner [0, 2] would be the 3rd column of the 1st row
    inner: Box<[CellState]>,
    pub column_count: usize,
    pub row_count: usize,
    pub to_win: u32,
}

impl Clone for Grid {
    fn clone(&self) -> Self {
        Grid {
            inner: self.inner.clone(),
            column_count: self.column_count,
            row_count: self.row_count,
            to_win: self.to_win,
        }
    }
}

impl Grid {
    pub fn new(row_count: usize, column_count: usize, streak_to_win: u32) -> Grid {
        Grid {
            inner: vec![CellState::Unset; column_count * row_count].into_boxed_slice(),
            column_count: column_count,
            row_count: row_count,
            to_win: streak_to_win,
        }
    }

    fn calc_index(&self, row: usize, column: usize) -> usize {
        if row >= self.row_count {
            panic!("index out of bounds: the row_count is {} but the row accessed is {}",
                   self.row_count, row)
        }
        if column >= self.column_count {
            panic!("index out of bounds: the column_count is {} but the column accessed is {}",
                   self.column_count, column)
        }

        column + row * self.column_count
    }

    fn get_mut_cell(&mut self, row: usize, column: usize) -> &mut CellState {
        &mut self.inner[self.calc_index(row, column)]
    }

    pub fn get_cell(&self, row: usize, column: usize) -> &CellState {
        &self.inner[self.calc_index(row, column)]
    }

    pub fn set_cell(&mut self, row: usize, column: usize, player: PlayerId) -> bool {
        let cell = self.get_mut_cell(row, column);
        match cell {
            &mut CellState::Unset => {
                *cell = CellState::Set(player);
                return true;
            },
            &mut CellState::Set(_) => return false,
        }
    }

    pub fn get_cells_with_state(&self, state: CellState) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for row_nr in 0 .. self.row_count {
            for col_nr in 0 .. self.column_count {
                if *self.get_cell(row_nr, col_nr) == state {
                    result.push((row_nr, col_nr));
                }
            }
        }
        result
    }

    pub fn pretty_print(&self) {
        for row_nr in 0 .. self.row_count {
            for cell_nr in 0 .. self.column_count {
                let string = match self.get_cell(row_nr, cell_nr) {
                    &CellState::Unset => format!("_"),
                    &CellState::Set(PlayerId(id)) => format!("{}", id),
                };
                print!("|{}", string);
            }
            println!("|");
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use ::game::{CellState, PlayerId};


    #[test]
    fn test_grid() {
        let grid = Grid::new(3, 3, 3);
        match grid.get_cell(0, 0) {
            &CellState::Unset => return,
            &CellState::Set(_) => panic!("Cell in a new grid is set even though it shouldn't."),
        }
    }

    #[test]
    fn test_set_cell() {
        let mut grid = Grid::new(3, 3, 3);
        if grid.set_cell(0, 0, PlayerId(1)) {
            match grid.get_cell(0, 0) {
                &CellState::Unset => panic!("Cell should be set after calling set_cell"),
                &CellState::Set(PlayerId(1)) => return,
                &CellState::Set(_) => panic!("Cell is set by the wrong player"),
            }
        } else {
            panic!("Cell could not be set although it shouldn't be set before");

        }
    }

    #[test]
    #[should_panic]
    fn test_set_cell_doesnt_work_out_of_bounds_rows() {
        let mut grid = Grid::new(4, 3, 3);
        grid.set_cell(4, 2, PlayerId(1));
    }

    #[test]
    #[should_panic]
    fn test_set_cell_doesnt_work_out_of_bounds_columns() {
        let mut grid = Grid::new(6, 7, 3);
        grid.set_cell(2, 8, PlayerId(1));
    }
}
