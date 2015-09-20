use ::game::Player;

struct ki {
    id: u32,
}

impl Player for ki {
    pub fn make_turn (&self, &mut grid: Grid) {
        unimplemented!();
    }
}
