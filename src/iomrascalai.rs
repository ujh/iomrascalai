/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
 *                                                                      *
 * This file is part of Iomrascálaí.                                    *
 *                                                                      *
 * Iomrascálaí is free software: you can redistribute it and/or modify  *
 * it under the terms of the GNU General Public License as published by *
 * the Free Software Foundation, either version 3 of the License, or    *
 * (at your option) any later version.                                  *
 *                                                                      *
 * Iomrascálaí is distributed in the hope that it will be useful,       *
 * but WITHOUT ANY WARRANTY; without even the implied warranty of       *
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the        *
 * GNU General Public License for more details.                         *
 *                                                                      *
 * You should have received a copy of the GNU General Public License    *
 * along with Iomrascálaí.  If not, see <http://www.gnu.org/licenses/>. *
 *                                                                      *
 ************************************************************************/
use board::{White, Black};
use std::io::stdio::stdin;

mod board;

fn main() {
  let mut b = board::Board::new(19, 6.5);
  let mut current_player = Black;
  let mut reader = stdin();
  let mut line =  "whatever".to_owned();

  while line.len() > 1 {
    print!("{} to play (Enter coordinates separated by space): ", current_player);

    line = reader.read_line().unwrap();

    let coords: Vec<u8> = line.as_slice().trim_chars('\n').split(' ').map(|s| from_str(s).unwrap()).collect();

    b = b.play(current_player, *coords.get(0), *coords.get(1));

    current_player = match current_player {
        Black => White,
        White => Black,
        _     => unreachable!()
    };

    b.show();
    b.show_chains();
  }
}
