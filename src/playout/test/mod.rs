/************************************************************************
 *                                                                      *
 * Copyright 2015 Thomas Poinsot, Urban Hafner                          *
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

use config::Config;

mod no_eyes;

fn config() -> Config {
    Config::default()
}

#[test]
fn factory_returns_no_self_atari_by_default() {
    let playout = super::factory(None, config());
    assert_eq!("no-self-atari", playout.playout_type());
}

#[test]
fn factory_returns_no_self_atari_when_given_any_string() {
    let playout = super::factory(Some(String::from_str("foo")), config());
    assert_eq!("no-self-atari", playout.playout_type());
}

#[test]
fn factory_returns_no_eyes_when_given_light() {
    let playout = super::factory(Some(String::from_str("light")), config());
    assert_eq!("no-eyes", playout.playout_type());
}
