use super::{CellState, Player};

const ROWS: usize = 10;
const COLUMNS: usize = 10;
const TO_WIN: u32 = 3;

type Row = [CellState; COLUMNS];
pub struct Grid {
    // inner [0, 2] would be the 3rd column of the 1st row
    inner: [Row; ROWS],
}

impl Grid {
    //TODO: should to_win and check_winner really be in Grid?
    pub fn new(rows: usize, columns: usize, to_win: usize) -> Grid {
        Grid {
            inner: [[CellState::Unset; COLUMNS]; ROWS]
        }
    }

    fn get_cell(&self, row: usize, column: usize) -> CellState {
        self.inner[row][column]
    }

    pub fn set_cell(&mut self, row: usize, column: usize, player: Player) -> bool {
        match self.get_cell(row, column) {
            CellState::Unset => {
                self.inner[row][column] = CellState::Set(player);
                return true;
            },
            CellState::Set(_) => return false,
        }
    }

    pub fn pretty_print(&self) {
        for row in self.inner.iter() {
            for cell in row.iter() {
                let string = match cell {
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

        for row in self.inner.iter() {
            for cell in row.iter() {
                Grid::check_cell(cell, &mut streak_player, &mut streak_length);
                if streak_length >= TO_WIN {
                    return Some(Player(streak_player));
                }
            }
        }
        None
    }

    fn check_vertical(&self) -> Option<Player> {
        let mut streak_player = 0;
        let mut streak_length = 0;

        for cellnr in 0 .. COLUMNS {
            for row in self.inner.iter() {
                let cell = &row[cellnr];
                Grid::check_cell(cell, &mut streak_player, &mut streak_length);
                if streak_length >= TO_WIN {
                    return Some(Player(streak_player));
                }
            }
        }
        None
    }

    fn check_diagonal(&self) -> Option<Player> {
        for rownr in 0 .. (COLUMNS - 1) {
            match self.check_diagonal_starting_at(rownr, 0) {
                None => continue,
                Some(Player(id)) => return Some(Player(id)),
            }
        }
        for colnr in 0 .. (ROWS - 1) {
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
        while (rownr < ROWS) & (colnr < COLUMNS) {
            let cell = &self.get_cell(rownr, colnr);
            Grid::check_cell(cell, &mut streak_player, &mut streak_length);
            if streak_length >= TO_WIN {
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
        let cell = grid.get_cell(0, 0);
        match cell {
            CellState::Unset => return,
            CellState::Set(_) => panic!("Cell in a new grid is set even though it shouldn't."),
        }
    }

    #[test]
    fn test_set_cell() {
        let mut grid = Grid::new(3, 3, 3);
        if grid.set_cell(0, 0, Player(1)) {
            match grid.get_cell(0, 0) {
                CellState::Unset => panic!("Cell should be set after calling set_cell"),
                CellState::Set(Player(1)) => return,
                CellState::Set(_) => panic!("Cell is set by the wrong player"),
            }
        } else {
            panic!("Cell could not be set although it shouldn't be set before");

        }
    }
}
