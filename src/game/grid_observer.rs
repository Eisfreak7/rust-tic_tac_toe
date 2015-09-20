use super::grid::Grid;
use super::{PlayerId, CellState};

pub fn check_winner(grid: &Grid) -> Option<PlayerId> {
    check_horizontal(&grid)
        .or(check_vertical(&grid))
        .or(check_diagonal(&grid))
}

fn check_horizontal(grid: &Grid) -> Option<PlayerId> {
    let mut streak_player = 0;
    let mut streak_length = 0;

    for row_nr in 0..grid.row_count {
        for cell_nr in 0 .. grid.column_count {
            let cell = grid.get_cell(row_nr, cell_nr);
            check_cell(&cell, &mut streak_player, &mut streak_length);
            if streak_length >= grid.to_win {
                return Some(PlayerId(streak_player));
            }
        }
    }
    None
}

fn check_vertical(grid: &Grid) -> Option<PlayerId> {
    let mut streak_player = 0;
    let mut streak_length = 0;

    for col_nr in 0 .. grid.column_count {
        for row_nr in 0..grid.row_count {
            let cell = &grid.get_cell(row_nr, col_nr);
            check_cell(cell, &mut streak_player, &mut streak_length);
            if streak_length >= grid.to_win {
                return Some(PlayerId(streak_player));
            }
        }
    }
    None
}

fn check_diagonal(grid: &Grid) -> Option<PlayerId> {
    for rownr in 0 .. grid.row_count {
        match check_diagonal_starting_at(&grid, rownr, 0) {
            None => continue,
            Some(PlayerId(id)) => return Some(PlayerId(id)),
        }
    }
    for colnr in 0 .. grid.column_count {
        match check_diagonal_starting_at(&grid, 0, colnr) {
            None => continue,
            Some(PlayerId(id)) => return Some(PlayerId(id)),
        }
    }
    None
}

fn check_diagonal_starting_at(grid: &Grid, startrow: usize, startcolumn: usize) -> Option<PlayerId> {
    let mut streak_player = 0;
    let mut streak_length = 0;

    let mut rownr = startrow;
    let mut colnr = startcolumn;
    while (rownr < grid.row_count) & (colnr < grid.column_count) {
        let cell = &grid.get_cell(rownr, colnr);
        check_cell(cell, &mut streak_player, &mut streak_length);
        if streak_length >= grid.to_win {
            return Some(PlayerId(streak_player));
        }
        colnr += 1;
        rownr += 1
    }
    None
}

fn check_cell(cell: &CellState, streak_player: &mut u32, streak_length: &mut u32) {
    match cell {
        &CellState::Unset => return,
        &CellState::Set(PlayerId(id)) if id == *streak_player => *streak_length += 1,
        &CellState::Set(PlayerId(id)) => {
            *streak_length = 1;
            *streak_player = id;
        }
    }
}




#[cfg(test)]
mod test {
    use super::*;
    use ::game::grid::Grid;
    use ::game::PlayerId;

    #[test]
    fn test_check_winner_horizontal_first_row() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(0, 1, PlayerId(1));
        grid.set_cell(0, 2, PlayerId(1));
        grid.set_cell(0, 3, PlayerId(1));
        grid.set_cell(0, 4, PlayerId(1));
        assert!(check_winner(&grid).is_some());
    }

    #[test]
    fn test_check_winner_horizontal_middle_row() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(3, 1, PlayerId(1));
        grid.set_cell(3, 2, PlayerId(1));
        grid.set_cell(3, 3, PlayerId(1));
        grid.set_cell(3, 4, PlayerId(1));
        assert!(check_winner(&grid).is_some());
    }

    #[test]
    fn test_check_winner_horizontal_last_row() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(8, 1, PlayerId(1));
        grid.set_cell(8, 2, PlayerId(1));
        grid.set_cell(8, 3, PlayerId(1));
        grid.set_cell(8, 4, PlayerId(1));
        assert!(check_winner(&grid).is_some());
    }

    #[test]
    fn test_check_winner_vertical_first_column() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(0, 0, PlayerId(1));
        grid.set_cell(1, 0, PlayerId(1));
        grid.set_cell(2, 0, PlayerId(1));
        grid.set_cell(3, 0, PlayerId(1));
        assert!(check_winner(&grid).is_some());
    }

    #[test]
    fn test_check_winner_vertical_middle_column() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(0, 3, PlayerId(1));
        grid.set_cell(1, 3, PlayerId(1));
        grid.set_cell(2, 3, PlayerId(1));
        grid.set_cell(3, 3, PlayerId(1));
        assert!(check_winner(&grid).is_some());
    }

    #[test]
    fn test_check_winner_vertical_last_column() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(0, 8, PlayerId(1));
        grid.set_cell(1, 8, PlayerId(1));
        grid.set_cell(2, 8, PlayerId(1));
        grid.set_cell(3, 8, PlayerId(1));
        assert!(check_winner(&grid).is_some());
    }

    #[test]
    fn test_check_winner_diagonal_corner_start() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(0, 0, PlayerId(1));
        grid.set_cell(1, 1, PlayerId(1));
        grid.set_cell(2, 2, PlayerId(1));
        grid.set_cell(3, 3, PlayerId(1));
        assert!(check_winner(&grid).is_some());
    }

    #[test]
    fn test_check_winner_diagonal_left_side_start() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(2, 0, PlayerId(1));
        grid.set_cell(3, 1, PlayerId(1));
        grid.set_cell(4, 2, PlayerId(1));
        grid.set_cell(5, 3, PlayerId(1));
        assert!(check_winner(&grid).is_some());
    }

    #[test]
    fn test_check_winner_diagonal_top_start() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(0, 2, PlayerId(1));
        grid.set_cell(1, 3, PlayerId(1));
        grid.set_cell(2, 4, PlayerId(1));
        grid.set_cell(3, 5, PlayerId(1));
        assert!(check_winner(&grid).is_some());
    }

    #[test]
    fn test_check_winner_diagonal_middle_start() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(1, 2, PlayerId(1));
        grid.set_cell(2, 3, PlayerId(1));
        grid.set_cell(3, 4, PlayerId(1));
        grid.set_cell(4, 5, PlayerId(1));
        assert!(check_winner(&grid).is_some());
    }

    #[test]
    fn test_check_winner_no_winner() {
        let mut grid = Grid::new(9, 9, 4);
        grid.set_cell(1, 8, PlayerId(1));
        grid.set_cell(2, 8, PlayerId(1));
        grid.set_cell(3, 8, PlayerId(2));
        grid.set_cell(4, 8, PlayerId(1));
        assert!(check_winner(&grid).is_none());
    }
}
