use ::game::grid::Grid;
use ::game::{PlayerId,GameState::*};
use ::game::grid_observer::check_winner;
use ::player::terminal::TerminalPlayer;
use ::player::ki::KiPlayer;
use ::player::Player;

mod game;
mod player;

const ROWS: usize = 3;
const COLUMNS: usize = 3;
const TO_WIN: u32 = 3;
const PLAYER_COUNT: usize = 2;

fn main() {
    let term_player_1 =  TerminalPlayer::new(1);
    let term_player_2 =  KiPlayer::new(2);
    let players: Vec<&Player> = vec![&term_player_1, &term_player_2];
    let mut cur_id: usize = 0;
    let mut grid = Grid::new(ROWS, COLUMNS, TO_WIN);

    loop {
        match check_winner(&grid) {
            Win(PlayerId(id)) => {
                println!("Congratiolations, Player {}. You Win!", id);
                return;
            },
            Mid => {
                players[cur_id].make_turn(&mut grid);
                if cur_id >= (PLAYER_COUNT - 1) {
                    cur_id = 0;
                } else {
                    cur_id += 1;
                }
            },
            Draw => {
                println!("Draw! You are equally good!");
                return;
            }
        }
    }
}

