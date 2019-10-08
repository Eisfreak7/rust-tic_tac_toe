#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub struct PlayerId(pub u32);

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum CellState {
    Set(PlayerId),
    Unset,
}

pub enum GameState {
    Win(PlayerId),
    Draw,
    Mid,
}

impl PartialEq for CellState {
    fn eq(&self, other: &CellState) -> bool {
        match *self {
            CellState::Unset => {
                match *other {
                    CellState::Unset => true,
                    _ => false,
                }
            },
            CellState::Set(PlayerId(own_id)) => {
                match *other {
                    CellState::Unset => false,
                    CellState::Set(PlayerId(other_id)) if other_id == own_id => true,
                    _ => false,
                }
            }
        }
    }
}


pub mod grid;
pub mod grid_observer;
