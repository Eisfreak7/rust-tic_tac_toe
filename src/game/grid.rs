use super::{CellState, Player};

pub struct Grid {
    // inner [0, 2] would be the 3rd column of the 1st row
    inner: Box<[CellState]>,
    column_count: usize,
    row_count: usize,
    to_win: u32,
}

impl Grid {
    //TODO: should to_win and check_winner really be in Grid?
    pub fn new(row_count: usize, column_count: usize, streak_to_win: u32) -> Grid {
        Grid {
            inner: vec![CellState::Unset; column_count * row_count].into_boxed_slice(),
            column_count: column_count,
            row_count: row_count,
            to_win: streak_to_win,
        }
    }

    fn calc_index(&self, row: usize, column: usize) -> usize {
        column + row * self.column_count
    }

    fn get_mut_cell(&mut self, row: usize, column: usize) -> &mut CellState {
        &mut self.inner[self.calc_index(row, column)]
    }

    fn get_cell(&self, row: usize, column: usize) -> &CellState {
        &self.inner[self.calc_index(row, column)]
    }

    pub fn set_cell(&mut self, row: usize, column: usize, player: Player) -> bool {
        let cell = self.get_mut_cell(row, column);
        match cell {
            &mut CellState::Unset => {
                *cell = CellState::Set(player);
                return true;
            },
            &mut CellState::Set(_) => return false,
        }
    }

    pub fn pretty_print(&self) {
        for row_nr in 0 .. self.row_count {
            for cell_nr in 0 .. self.column_count {
                let string = match self.get_cell(row_nr, cell_nr) {
                    &CellState::Unset => format!("_"),
                    &CellState::Set(Player(id)) => format!("{}", id),
                };
                print!("|{}", string);
            }
            println!("|");
        }
    }

    pub fn check_winner(&self) -> Option<Player> {
        self.check_horizontal()
            .or(self.check_vertical())
            .or(self.check_diagonal())
    }

    fn check_horizontal(&self) -> Option<Player> {
        let mut streak_player = 0;
        let mut streak_length = 0;

        for row_nr in 0..self.row_count {
            for cell_nr in 0 .. self.column_count {
                let cell = self.get_cell(row_nr, cell_nr);
                Grid::check_cell(&cell, &mut streak_player, &mut streak_length);
                if streak_length >= self.to_win {
                    return Some(Player(streak_player));
                }
            }
        }
        None
    }

    fn check_vertical(&self) -> Option<Player> {
        let mut streak_player = 0;
        let mut streak_length = 0;

        for col_nr in 0 .. self.column_count {
            for row_nr in 0..self.row_count {
                let cell = &self.get_cell(row_nr, col_nr);
                Grid::check_cell(cell, &mut streak_player, &mut streak_length);
                if streak_length >= self.to_win {
                    return Some(Player(streak_player));
                }
            }
        }
        None
    }

    fn check_diagonal(&self) -> Option<Player> {
        for rownr in 0 .. self.row_count {
            match self.check_diagonal_starting_at(rownr, 0) {
                None => continue,
                Some(Player(id)) => return Some(Player(id)),
            }
        }
        for colnr in 0 .. self.column_count {
            match self.check_diagonal_starting_at(0, colnr) {
                None => continue,
                Some(Player(id)) => return Some(Player(id)),
            }
        }
        None
    }

    fn check_diagonal_starting_at(&self, startrow: usize, startcolumn: usize) -> Option<Player> {
        let mut streak_player = 0;
        let mut streak_length = 0;

        let mut rownr = startrow;
        let mut colnr = startcolumn;
        while (rownr < self.row_count) & (colnr < self.column_count) {
            let cell = &self.get_cell(rownr, colnr);
            Grid::check_cell(cell, &mut streak_player, &mut streak_length);
            if streak_length >= self.to_win {
                return Some(Player(streak_player));
            }
            colnr += 1;
            rownr += 1
        }
        None
    }

    fn check_cell(cell: &CellState, streak_player: &mut u32, streak_length: &mut u32) {
        match cell {
            &CellState::Unset => return,
            &CellState::Set(Player(id)) if id == *streak_player => *streak_length += 1,
            &CellState::Set(Player(id)) => {
                *streak_length = 1;
                *streak_player = id;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ::game::{CellState, Player};

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
        if grid.set_cell(0, 0, Player(1)) {
            match grid.get_cell(0, 0) {
                &CellState::Unset => panic!("Cell should be set after calling set_cell"),
                &CellState::Set(Player(1)) => return,
                &CellState::Set(_) => panic!("Cell is set by the wrong player"),
            }
        } else {
            panic!("Cell could not be set although it shouldn't be set before");

        }
    }
}
