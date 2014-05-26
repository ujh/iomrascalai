use board::Color;
use board::coord::Coord;

#[deriving(Clone, Eq)]
pub struct Chain {
    pub id: uint,
    pub color: Color,
    coords: Vec<Coord>
}

impl Chain {
    pub fn new(id: uint, color: Color) -> Chain {
        Chain {coords: Vec::new(), color: color, id: id}
    }

    pub fn add_stone(&mut self, coord: Coord) {
        self.coords.push(coord);
    }

    pub fn merge(&mut self, c: &Chain) {
        for coord in c.coords.iter() {
            self.coords.push(*coord);
        }
    }

    pub fn coords<'a>(&'a self) -> &'a Vec<Coord> {
        &self.coords
    }

    pub fn show(&self) -> String {
        self.coords
            .iter()
            .fold(String::from_owned_str(format!("{}, {}: ", self.id, self.color)), |s, c| s.append(format!("{},{}|", c.col, c.row).as_slice()))
    }
}