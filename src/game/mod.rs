#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct PlayerId(pub u32);

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum CellState {
    Set(PlayerId),
    Unset,
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

    fn ne(&self, other: &CellState) -> bool {
        !self.eq(other)
    }
}


pub mod grid;
pub mod grid_observer;
