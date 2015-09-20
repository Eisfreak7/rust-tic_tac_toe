use super::grid::Grid;
use super::{Player, CellState};

pub fn check_winner(grid: &Grid) -> Option<Player> {
    check_horizontal(&grid)
        .or(check_vertical(&grid))
        .or(check_diagonal(&grid))
}

fn check_horizontal(grid: &Grid) -> Option<Player> {
    let mut streak_player = 0;
    let mut streak_length = 0;

    for row_nr in 0..grid.row_count {
        for cell_nr in 0 .. grid.column_count {
            let cell = grid.get_cell(row_nr, cell_nr);
            check_cell(&cell, &mut streak_player, &mut streak_length);
            if streak_length >= grid.to_win {
                return Some(Player(streak_player));
            }
        }
    }
    None
}

fn check_vertical(grid: &Grid) -> Option<Player> {
    let mut streak_player = 0;
    let mut streak_length = 0;

    for col_nr in 0 .. grid.column_count {
        for row_nr in 0..grid.row_count {
            let cell = &grid.get_cell(row_nr, col_nr);
            check_cell(cell, &mut streak_player, &mut streak_length);
            if streak_length >= grid.to_win {
                return Some(Player(streak_player));
            }
        }
    }
    None
}

fn check_diagonal(grid: &Grid) -> Option<Player> {
    for rownr in 0 .. grid.row_count {
        match check_diagonal_starting_at(&grid, rownr, 0) {
            None => continue,
            Some(Player(id)) => return Some(Player(id)),
        }
    }
    for colnr in 0 .. grid.column_count {
        match check_diagonal_starting_at(&grid, 0, colnr) {
            None => continue,
            Some(Player(id)) => return Some(Player(id)),
        }
    }
    None
}

fn check_diagonal_starting_at(grid: &Grid, startrow: usize, startcolumn: usize) -> Option<Player> {
    let mut streak_player = 0;
    let mut streak_length = 0;

    let mut rownr = startrow;
    let mut colnr = startcolumn;
    while (rownr < grid.row_count) & (colnr < grid.column_count) {
        let cell = &grid.get_cell(rownr, colnr);
        check_cell(cell, &mut streak_player, &mut streak_length);
        if streak_length >= grid.to_win {
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




#[cfg(test)]
mod test {
    use super::*;
    use ::game::grid::Grid;
    use ::game::Player;

    #[test]
    fn test_check_winner_horizontal_first_row() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(0, 1, Player(1));
        grid.set_cell(0, 2, Player(1));
        grid.set_cell(0, 3, Player(1));
        grid.set_cell(0, 4, Player(1));
        assert!(check_winner(&grid).is_some());
    }

    #[test]
    fn test_check_winner_horizontal_middle_row() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(3, 1, Player(1));
        grid.set_cell(3, 2, Player(1));
        grid.set_cell(3, 3, Player(1));
        grid.set_cell(3, 4, Player(1));
        assert!(check_winner(&grid).is_some());
    }

    #[test]
    fn test_check_winner_horizontal_last_row() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(8, 1, Player(1));
        grid.set_cell(8, 2, Player(1));
        grid.set_cell(8, 3, Player(1));
        grid.set_cell(8, 4, Player(1));
        assert!(check_winner(&grid).is_some());
    }

    #[test]
    fn test_check_winner_vertical_first_column() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(0, 0, Player(1));
        grid.set_cell(1, 0, Player(1));
        grid.set_cell(2, 0, Player(1));
        grid.set_cell(3, 0, Player(1));
        assert!(check_winner(&grid).is_some());
    }

    #[test]
    fn test_check_winner_vertical_middle_column() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(0, 3, Player(1));
        grid.set_cell(1, 3, Player(1));
        grid.set_cell(2, 3, Player(1));
        grid.set_cell(3, 3, Player(1));
        assert!(check_winner(&grid).is_some());
    }

    #[test]
    fn test_check_winner_vertical_last_column() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(0, 8, Player(1));
        grid.set_cell(1, 8, Player(1));
        grid.set_cell(2, 8, Player(1));
        grid.set_cell(3, 8, Player(1));
        assert!(check_winner(&grid).is_some());
    }

    #[test]
    fn test_check_winner_diagonal_corner_start() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(0, 0, Player(1));
        grid.set_cell(1, 1, Player(1));
        grid.set_cell(2, 2, Player(1));
        grid.set_cell(3, 3, Player(1));
        assert!(check_winner(&grid).is_some());
    }

    #[test]
    fn test_check_winner_diagonal_left_side_start() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(2, 0, Player(1));
        grid.set_cell(3, 1, Player(1));
        grid.set_cell(4, 2, Player(1));
        grid.set_cell(5, 3, Player(1));
        assert!(check_winner(&grid).is_some());
    }

    #[test]
    fn test_check_winner_diagonal_top_start() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(0, 2, Player(1));
        grid.set_cell(1, 3, Player(1));
        grid.set_cell(2, 4, Player(1));
        grid.set_cell(3, 5, Player(1));
        assert!(check_winner(&grid).is_some());
    }

    #[test]
    fn test_check_winner_diagonal_middle_start() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(1, 2, Player(1));
        grid.set_cell(2, 3, Player(1));
        grid.set_cell(3, 4, Player(1));
        grid.set_cell(4, 5, Player(1));
        assert!(check_winner(&grid).is_some());
    }

    #[test]
    fn test_check_winner_no_winner() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(1, 8, Player(1));
        grid.set_cell(2, 8, Player(1));
        grid.set_cell(3, 8, Player(2));
        grid.set_cell(4, 8, Player(1));
        assert!(check_winner(&grid).is_none());
    }
}
