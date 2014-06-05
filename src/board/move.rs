use board::{Color, Black, White};
use board::coord::Coord;

#[deriving(Show)]
pub enum Move {
    Play(Color, u8, u8),
    Pass(Color)
}

impl Move {
    pub fn from_gtp(color: &str, vertex: &str) -> Move {
        let lower_case_color: String = color.chars().map(|c| c.to_lowercase()).collect();
        let c = match lower_case_color.as_slice() {
            "w" | "white" => White,
            "b" | "black" => Black,
            _             => fail!("Couldn't read color")
        };

        let lower_case_vertex: String = vertex.chars().map(|c| c.to_lowercase()).collect();
        let col = lower_case_vertex.as_slice().char_at(0) as u8 - 'a' as u8 + 1;
        let row = from_str::<u8>(lower_case_vertex.as_slice().slice(1, 2)).expect("you must enter a valid coord (1 < c < 256)");

        Play(c, col, row)
    }

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