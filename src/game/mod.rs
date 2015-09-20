#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Player(pub u32);

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum CellState {
    Set(Player),
    Unset,
}


pub mod grid;
