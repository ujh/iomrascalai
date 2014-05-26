use board::Color;
use board::coord::Coord;

#[deriving(Clone, Eq)]
pub struct Chain {
    pub id   : uint,
    pub color: Color,
    pub libs : uint,
    coords   : Vec<Coord>
}

impl Chain {
    pub fn new(id: uint, color: Color) -> Chain {
        Chain {coords: Vec::new(), color: color, id: id, libs: 1}
    }

    pub fn add_stone(&mut self, coord: Coord, new_libs: uint) {
        self.libs += new_libs - 1;
        self.coords.push(coord);
    }

    pub fn merge(&mut self, c: &Chain) {
        self.libs += c.libs - 1;
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
            .fold(String::from_owned_str(format!("{}| {}, {} libs: ", self.id, self.color, self.libs)), |s, c| s.append(format!("{},{}|", c.col, c.row).as_slice()))
    }
}