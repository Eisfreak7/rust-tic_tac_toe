use ::player::Player;
use ::game::grid::Grid;
use ::game::grid_observer;
use ::game::{CellState, PlayerId};

pub struct KiPlayer {
    id: u32,
}

impl KiPlayer {
    pub fn new(id: u32) -> KiPlayer {
        KiPlayer {
            id: id,
        }
    }
}

impl KiPlayer {
    fn evaluate_game(&self, grid: &Grid) -> Option<i32> {
        match grid_observer::check_winner(grid) {
            None => {
                if grid.get_cells_with_state(CellState::Unset).is_empty() {
                    Some(0)
                } else {
                    None
                }
            },
            Some(PlayerId(id)) if id == self.id => Some(100),
            _ => Some(-100)
        }
    }

}

impl Player for KiPlayer {
    fn make_turn (&self, grid: &mut Grid) {
        match self.evaluate_game(grid) {
            Some(value) => println!("This game looks like a {} to me.", value),
            None => println!("I can't evaluate this game yet."),
        }

        for row_nr in 0 .. grid.row_count {
            for col_nr in 0 .. grid.column_count {
                match grid.get_cell(row_nr, col_nr) {
                    &CellState::Unset => {
                        grid.set_cell(row_nr, col_nr, PlayerId(self.id));
                        return;
                    },
                    &CellState::Set(_) => continue,
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ::player::Player;
    use ::game::{CellState, PlayerId};
    use ::game::grid::Grid;


    #[test]
    fn ki_makes_any_turn() {
        const KI_ID: u32 = 1;
        let mut grid = Grid::new(1, 1, 1);
        let ki = KiPlayer::new(KI_ID);
        ki.make_turn(&mut grid);
        match grid.get_cell(0, 0) {
            &CellState::Unset => panic!("The ki didn't do anything."),
            &CellState::Set(PlayerId(KI_ID)) => return,
            &CellState::Set(PlayerId(id)) =>
                panic!("The ki with the {} made a turn for player {}.", KI_ID, id),
        }
    }
}
