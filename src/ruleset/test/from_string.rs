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

use std::str::FromStr;

#[test]
fn parses_tromp_taylor() {
    assert_eq!(Ok(AnySizeTrompTaylor), Ruleset::from_str("tromp-taylor"));
}

#[test]
fn parses_cgos() {
    assert_eq!(Ok(CGOS), Ruleset::from_str("cgos"));
}

#[test]
fn parses_chinese() {
    assert_eq!(Ok(KgsChinese), Ruleset::from_str("chinese"));
}

#[test]
fn parses_minimal() {
    assert_eq!(Ok(Minimal), Ruleset::from_str("minimal"));
}

#[test]
fn errors_with_unknown() {
    assert_eq!(Err(String::from("Unknown ruleset 'unknown'")), Ruleset::from_str("unknown"));
}
