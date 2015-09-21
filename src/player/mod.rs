pub mod terminal;
pub mod ki;

use ::game::grid::Grid;

pub trait Player {
    fn make_turn (&self, grid: &mut Grid);
}
