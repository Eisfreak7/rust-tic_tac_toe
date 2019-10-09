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

impl GameState {
    fn is_some(&self) -> bool {
        match *self {
            GameState::Draw => false,
            GameState::Mid => false,
            _ => true,
        } 
    }

    fn is_none(&self) -> bool {
        !self.is_some()
    }
}


pub mod grid;
pub mod grid_observer;
