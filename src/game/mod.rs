#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct PlayerId(pub u32);

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum CellState {
    Set(PlayerId),
    Unset,
}


pub mod grid;
pub mod grid_observer;
