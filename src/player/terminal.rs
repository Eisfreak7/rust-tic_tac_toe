use std::io;
use super::Player;
use ::game::PlayerId;
use ::game::grid::Grid;

pub struct TerminalPlayer {
    id: u32,
}

impl TerminalPlayer {
    pub fn new(id: u32) -> TerminalPlayer {
        println!("Welcome, Player {}!", id);
        println!("This is a simple implementation of the classical game 'Tic-Tac-Toe'.");
        println!("If you are asked for input, you should enter it in the form 'row column'");
        println!("Row and column numeration starts at 0.");
        println!("Example: To set the cell at row 0 and column 2, enter '0 2'");

        TerminalPlayer {
            id: id,
        }
    }
}

impl Player for TerminalPlayer {
    fn make_turn (&self, grid: &mut Grid) {
        println!("\nCurrent state:");
        grid.pretty_print();
        println!("Player {}, what is your turn?", self.id);

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

        grid.set_cell(row, column, PlayerId(self.id));
    }
}
