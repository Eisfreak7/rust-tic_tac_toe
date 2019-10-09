use ::player::Player;
use ::game::grid::Grid;
use ::game::grid_observer;
use ::game::{CellState, PlayerId, GameState};

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
pub enum GameEvaluation {
    Win,
    Lose,
    Draw,
}

#[derive(Clone)]
struct Move {
    pub row: usize,
    pub column: usize,
}

impl Move {
    fn new(row: usize, column: usize) -> Move {
        Move {
            row: row,
            column: column,
        }
    }
}

//TODO: cache evaluations and just look at differences
//TODO: benchmark
pub fn evaluate_game(grid: &Grid, perspective: PlayerId) -> Option<GameEvaluation> {
    match grid_observer::check_winner(grid) {
        GameState::Mid => None,
        GameState::Win(winner) => {
            if winner == perspective {
                Some(GameEvaluation::Win)
            } else {
                Some(GameEvaluation::Lose)
            }
        },
        GameState::Draw => Some(GameEvaluation::Draw)
    }
}

//TODO: Implement different difficulties
// less difficultiy: lower depth limit
// if no move can be found within the depth limit, chose a random move
// maybe the depth limit can be a percentage of the maximum depth?
//TODO: Cleaner return value. Maybe a GameEnd enum with Draw or Winner(PlayerId) as options?
fn minimax(grid: &Grid, current_player: PlayerId, other_player: PlayerId, depth: u32)
    -> (GameEvaluation, Option<Move>, u32) {
    let evaluation = evaluate_game(grid, current_player);

    if evaluation.is_some() {
        (evaluation.unwrap(), None, depth)
    } else {
        let mut wins: Vec<(Move, u32)> = Vec::new();
        let mut draws: Vec<(Move, u32)> = Vec::new();
        let mut loses: Vec<(Move, u32)> = Vec::new();

        for (row, col) in grid.get_cells_with_state(CellState::Unset) {
            //TODO: Use multiple threads
            let mut new_grid = grid.clone();
            new_grid.set_cell(row, col, current_player);
            let (evaluation, _, depth) = minimax(&new_grid, other_player, current_player, depth + 1);
            match evaluation {
                //TODO: Stop mixing up perspectives
                GameEvaluation::Lose => wins.push((Move::new(row, col), depth)),
                GameEvaluation::Draw => draws.push((Move::new(row, col), depth)),
                GameEvaluation::Win => loses.push((Move::new(row, col), depth)),
            }
        }

        if !wins.is_empty() {
            let (mov, depth) = wins[0].clone();
            (GameEvaluation::Win, Some(mov), depth)
        } else if !draws.is_empty() {
            let (mov, depth) = draws[0].clone();
            (GameEvaluation::Draw, Some(mov), depth)
        } else if !loses.is_empty() {
            let (mov, depth) = loses[0].clone();
            (GameEvaluation::Lose, Some(mov), depth)
        } else {
            panic!("No possible move, even though the game souldn't be finished.");
        }
    }

    }

impl Player for KiPlayer {
    fn make_turn (&self, grid: &mut Grid) {
        //TODO: Proper way to determine other player (Array of players?)
        let other_id = if self.id == 1 {2} else {1};
        let (_, position, _) = minimax(grid, PlayerId(self.id), PlayerId(other_id), 0);
        // TODO: don't use unwrap
        let position = position.unwrap();
        grid.set_cell(position.row, position.column, PlayerId(self.id));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ::player::Player;
    use ::game::{CellState, PlayerId};
    use ::game::grid::Grid;
    use ::game::grid_observer;


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

    // TODO: Make tests work
    // #[test]
    // fn ki_makes_winning_move() {
    //     const KI_ID: u32 = 1;
    //     let mut grid = Grid::new(10, 10, 2);
    //     grid.set_cell(5, 5, PlayerId(KI_ID));
    //     KiPlayer::new(KI_ID).make_turn(&mut grid);

    //     match grid_observer::check_winner(&grid) {
    //         None => panic!("The Ki didn't make the obvious winning move"),
    //         Some(PlayerId(id)) if id != KI_ID => panic!("The Ki somehow managed to lose"),
    //         _ => {},
    //     }
    // }

    // #[test]
    // fn ki_blocks() {
    //     const KI_ID: u32 = 1;
    //     const OPPONENT_ID: u32 = 2;
    //     let mut grid = Grid::new(3, 3, 3);
    //     grid.set_cell(1, 0, PlayerId(OPPONENT_ID));
    //     grid.set_cell(1, 1, PlayerId(OPPONENT_ID));
    //     let ki = KiPlayer::new(KI_ID);
    //     ki.make_turn(&mut grid);

    //     grid.pretty_print();
    //     assert_eq!(CellState::Set(PlayerId(KI_ID)), *grid.get_cell(1, 2));
    // }

    #[test]
    fn ki_wins_without_active_opponent() {
        const KI_ID: u32 = 1;
        let mut grid = Grid::new(3, 3, 3);
        let ki = KiPlayer::new(KI_ID);
        for _ in 0 .. grid.to_win {
            ki.make_turn(&mut grid);
        }

        match grid_observer::check_winner(&grid) {
            GameState::Win(PlayerId(id)) if id != KI_ID => panic!("The Ki somehow managed to lose"),
            _ => panic!("The Ki can't even win without an opponent."),
        }
    }

    #[test]
    fn test_game_evaluation_undetermined() {
        const KI_ID: u32 = 1;
        let mut grid = Grid::new(3, 3, 3);
        assert!(evaluate_game(&grid, PlayerId(KI_ID)).is_none());
        grid.set_cell(0, 0, PlayerId(KI_ID));
        assert!(evaluate_game(&grid, PlayerId(KI_ID)).is_none());
    }

    #[test]
    fn test_game_evaluation_win() {
        const KI_ID: u32 = 1;
        let mut grid = Grid::new(3, 3, 3);
        grid.set_cell(0, 0, PlayerId(KI_ID));
        grid.set_cell(0, 1, PlayerId(KI_ID));
        grid.set_cell(0, 2, PlayerId(KI_ID));
        assert_eq!(GameEvaluation::Win, evaluate_game(&grid, PlayerId(KI_ID)).unwrap());
    }

    #[test]
    fn test_game_evaluation_lose() {
        const KI_ID: u32 = 1;
        const OPPONENT_ID: u32 = 2;
        let mut grid = Grid::new(3, 3, 3);
        grid.set_cell(0, 0, PlayerId(OPPONENT_ID));
        grid.set_cell(0, 1, PlayerId(OPPONENT_ID));
        grid.set_cell(0, 2, PlayerId(OPPONENT_ID));
        assert_eq!(GameEvaluation::Lose, evaluate_game(&grid, PlayerId(KI_ID)).unwrap());
    }
}
