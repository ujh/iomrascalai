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

mod no_eyes;

#[test]
fn factory_returns_no_self_atari_by_default() {
    let playout = super::factory(None);
    assert_eq!("NoSelfAtariPlayout", playout.playout_type());
}

#[test]
fn factory_returns_no_self_atari_when_given_any_string() {
    let playout = super::factory(Some(String::from_str("foo")));
    assert_eq!("NoSelfAtariPlayout", playout.playout_type());
}

#[test]
fn factory_returns_no_eyes_when_given_light() {
    let playout = super::factory(Some(String::from_str("light")));
    assert_eq!("NoEyesPlayout", playout.playout_type());
}
