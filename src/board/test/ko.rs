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
use board::Board;
use board::IllegalMove;
use board::Play;
use board::White;
use ruleset::AnySizeTrompTaylor;
use sgf::Parser;

#[test]
fn replaying_directly_on_a_ko_point_should_be_illegal() {
    let mut b = Board::new(19, 6.5, AnySizeTrompTaylor);

    b.play(Play(Black, 4, 4));
    b.play(Play(White, 5, 4));
    b.play(Play(Black, 3, 3));
    b.play(Play(White, 4, 3));
    b.play(Play(Black, 3, 5));
    b.play(Play(White, 4, 5));
    b.play(Play(Black, 2, 4));
    b.play(Play(White, 3, 4));

    let ko = b.play(Play(Black, 4, 4));
    match ko {
        Err(e) => assert_eq!(e, IllegalMove::Ko),
        Ok(_)  => panic!("Error expected")
    }
}

#[test]
fn positional_super_ko_should_be_legal() {
    let parser    = Parser::from_path(Path::new("fixtures/sgf/positional-superko.sgf"));
    let game      = parser.game().unwrap();
    let mut board = game.board();
    let super_ko = board.play(Play(White, 2, 9));
    assert_eq!(super_ko.is_ok(), true);
}

#[test]
fn recapture_that_captures_several_stones_isnt_ko() {
    let parser    = Parser::from_path(Path::new("fixtures/sgf/recapture-but-not-ko.sgf"));
    let game      = parser.game().unwrap();
    let mut board = game.board();
    let result = board.play(Play(White, 1, 6)); // a6
    match result {
        Err(e) => panic!("Unexpected error {:?}", e),
        Ok(_)  => {}
    }
}
