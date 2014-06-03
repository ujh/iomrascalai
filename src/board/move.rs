use board::Color;
use board::coord::Coord;

pub enum Move {
    Play(Color, u8, u8),
    Pass(Color)
}

impl Move {
    pub fn color(&self) -> Color {
        match self {
            &Play(c, _, _) => c,
            &Pass(c)       => c
        }
    }

    pub fn coords(&self) -> Coord {
        match self {
            &Play(_, col, row) => Coord::new(col, row),
            &Pass(_)           => fail!("You have tried to get the coords() of a Pass move")
        }
    }

    pub fn is_pass(&self) -> bool {
        match self {
            &Pass(_) => true,
            _        => false
        }
    }
}