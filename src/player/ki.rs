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

#[derive(Debug)]
#[derive(PartialEq)]
enum GameEvaluation {
    Win,
    Lose,
    Draw,
    Undetermined,
}

impl KiPlayer {
    fn evaluate_game(&self, grid: &Grid) -> GameEvaluation {
        match grid_observer::check_winner(grid) {
            None => {
                if grid.get_cells_with_state(CellState::Unset).is_empty() {
                    GameEvaluation::Draw
                } else {
                    GameEvaluation::Undetermined
                }
            },
            Some(PlayerId(id)) if id == self.id => GameEvaluation::Win,
            _ => GameEvaluation::Lose,
        }
    }

}

impl Player for KiPlayer {
    fn make_turn (&self, grid: &mut Grid) {
        println!("This game looks like a {:?} to me.", self.evaluate_game(grid));

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
    use super::GameEvaluation;
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

    #[test]
    fn test_game_evaluation_undetermined() {
        const KI_ID: u32 = 1;
        let mut grid = Grid::new(3, 3, 3);
        let ki = KiPlayer::new(KI_ID);
        assert_eq!(GameEvaluation::Undetermined, ki.evaluate_game(&grid));
        grid.set_cell(0, 0, PlayerId(KI_ID));
        assert_eq!(GameEvaluation::Undetermined, ki.evaluate_game(&grid));
    }

    #[test]
    fn test_game_evaluation_win() {
        const KI_ID: u32 = 1;
        let mut grid = Grid::new(3, 3, 3);
        let ki = KiPlayer::new(KI_ID);
        grid.set_cell(0, 0, PlayerId(KI_ID));
        grid.set_cell(0, 1, PlayerId(KI_ID));
        grid.set_cell(0, 2, PlayerId(KI_ID));
        assert_eq!(GameEvaluation::Win, ki.evaluate_game(&grid));
    }

    #[test]
    fn test_game_evaluation_lose() {
        const KI_ID: u32 = 1;
        const OPPONENT_ID: u32 = 2;
        let mut grid = Grid::new(3, 3, 3);
        let ki = KiPlayer::new(KI_ID);
        grid.set_cell(0, 0, PlayerId(OPPONENT_ID));
        grid.set_cell(0, 1, PlayerId(OPPONENT_ID));
        grid.set_cell(0, 2, PlayerId(OPPONENT_ID));
        assert_eq!(GameEvaluation::Lose, ki.evaluate_game(&grid));
    }
}
