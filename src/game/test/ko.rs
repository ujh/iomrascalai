/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
 * Copyright 2015 Urban Hafner                                          *
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

#![cfg(test)]

use board::Black;
use board::IllegalMove;
use board::Play;
use board::White;
use game::Game;
use ruleset::AnySizeTrompTaylor;
use sgf::Parser;

#[test]
fn replaying_directly_on_a_ko_point_should_be_illegal() {
    let mut g = Game::new(19, 6.5, AnySizeTrompTaylor);

    g = g.play(Play(Black, 4, 4)).unwrap();
    g = g.play(Play(White, 5, 4)).unwrap();
    g = g.play(Play(Black, 3, 3)).unwrap();
    g = g.play(Play(White, 4, 3)).unwrap();
    g = g.play(Play(Black, 3, 5)).unwrap();
    g = g.play(Play(White, 4, 5)).unwrap();
    g = g.play(Play(Black, 2, 4)).unwrap();
    g = g.play(Play(White, 3, 4)).unwrap();

    let ko = g.play(Play(Black, 4, 4));
    assert!(ko.is_err());
    assert_eq!(ko.unwrap_err(), IllegalMove::Ko);
}

#[test]
fn positional_super_ko_should_be_illegal() {
    let parser = Parser::from_path(Path::new("fixtures/sgf/positional-superko.sgf"));
    let game   = parser.game().unwrap();
    let super_ko = game.play(Play(White, 2, 9));
    assert!(super_ko.is_err());
    assert_eq!(super_ko.unwrap_err(), IllegalMove::SuperKo);
}
