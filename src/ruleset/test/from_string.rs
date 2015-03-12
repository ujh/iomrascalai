/************************************************************************
 *                                                                      *
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

use ruleset::AnySizeTrompTaylor;
use ruleset::CGOS;
use ruleset::KgsChinese;
use ruleset::Minimal;
use ruleset::Ruleset;

#[test]
fn parses_tromp_taylor() {
    assert_eq!(AnySizeTrompTaylor, Ruleset::from_string(String::from_str("tromp-taylor")));
}

#[test]
fn parses_cgos() {
    assert_eq!(CGOS, Ruleset::from_string(String::from_str("cgos")));
}

#[test]
fn parses_chinese() {
    assert_eq!(KgsChinese, Ruleset::from_string(String::from_str("chinese")));
}

#[test]
fn parses_minimal() {
    assert_eq!(Minimal, Ruleset::from_string(String::from_str("minimal")));
}

#[test]
#[should_panic]
fn fails_with_unknown() {
    Ruleset::from_string(String::from_str("unknown"));
}
