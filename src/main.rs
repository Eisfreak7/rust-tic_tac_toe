mod game;

use ::game::grid::Grid;
use ::game::Player;
use ::game::grid_observer::check_winner;
use std::io;


const ROWS: usize = 3;
const COLUMNS: usize = 3;
const TO_WIN: u32 = 3;
const PLAYER_COUNT: u32 = 2;

fn main() {
    println!("This is a simple implementation of the classical game 'Tic-Tac-Toe'.");
    println!("If you are asked for input, you should enter it in the form 'row column'");
    println!("Row and column numeration starts at 0.");
    println!("Example: To set the cell at row 0 and column 2, enter '0 2'");
    let mut grid = Grid::new(ROWS, COLUMNS, TO_WIN);

    let mut current_player = 1;
    loop {
        println!("\nCurrent state:");
        grid.pretty_print();

        match check_winner(&grid) {
            Some(Player(id)) => {
                println!("Congratiolations, Player {}. You Win!", id);
                return;
            },
            None => {
                println!("Player {}, what is your turn?", current_player);

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .ok()
                    .expect("failed to read line");

                let split: Vec<_> = input.split_whitespace().collect();
                assert_eq!(2, split.len());

                let row: usize = split[0]
                    .parse()
                    .ok()
                    .expect("failed to parse the input");

                let column: usize = split[1]
                    .parse()
                    .ok()
                    .expect("failed to parse the input");

                grid.set_cell(row, column, Player(current_player));

                if current_player >=  PLAYER_COUNT {
                    current_player = 1;
                } else {
                    current_player += 1;
                }
            }
        }
    }
}

