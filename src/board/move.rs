use board::Color;
use board::coord::Coord;

#[deriving(Show)]
pub enum Move {
    Play(Color, u8, u8),
    Pass(Color)
}

impl Move {
    pub fn from_gtp(gtp_color: &str, gtp_vertex: &str) -> Move {
        let color = Color::from_gtp(gtp_color);
        let coord = Coord::from_gtp(gtp_vertex);

        Play(color, coord.col, coord.row)
    }

    pub fn to_gtp(&self) -> String {
        match self {
            &Pass(_)           => String::from_str("pass"),
            &Play(_, col, row) => Coord::new(col, row).to_gtp()
        }
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