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

use super::AnySizeTrompTaylor;
use super::CGOS;
use super::KgsChinese;
use super::Minimal;

#[test]
fn tromp_taylor_allows_suicide() {
    assert_eq!(true, AnySizeTrompTaylor.suicide_allowed());
}

#[test]
fn tromp_taylor_forbids_a_player_playing_twice() {
    assert_eq!(false, AnySizeTrompTaylor.same_player());
}

#[test]
fn tromp_taylor_forbids_game_over_play() {
    assert_eq!(false, AnySizeTrompTaylor.game_over_play());
}

#[test]
fn cgos_forbids_suicide() {
    assert_eq!(false, CGOS.suicide_allowed());
}

#[test]
fn cgos_forbids_a_player_playing_twice() {
    assert_eq!(false, CGOS.same_player());
}

#[test]
fn cgos_forbids_game_over_play() {
    assert_eq!(false, CGOS.game_over_play());
}

#[test]
fn kgs_chinese_forbids_suicide() {
    assert_eq!(false, KgsChinese.suicide_allowed());
}

#[test]
fn kgs_chinese_forbids_a_player_playing_twice() {
    assert_eq!(false, KgsChinese.same_player());
}

#[test]
fn kgs_chinese_forbids_game_over_play() {
    assert_eq!(false, KgsChinese.game_over_play());
}

#[test]
fn minimal_allows_suicide() {
    assert_eq!(true, Minimal.suicide_allowed());
}

#[test]
fn minimal_allows_a_player_playing_twice() {
    assert_eq!(true, Minimal.same_player());
}

#[test]
fn minimal_allows_game_over_play() {
    assert_eq!(true, Minimal.game_over_play());
}
