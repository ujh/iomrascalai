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

use getopts::Options;
use ruleset::Ruleset;
use super::Config;

#[test]
fn fail_if_ladder_and_atari_are_in_conflict() {
    let mut config = Config::default();
    config.playout.atari_check = false;
    config.playout.ladder_check = true;
    assert!(config.check().is_err());
}

#[test]
fn playout_aftermath_under_cgos_rules_by_default() {
    let mut config = Config::default();
    config.ruleset = Ruleset::CGOS;
    let args = vec!();
    let mut opts = Options::new();
    config.setup(&mut opts);
    let matches = opts.parse(args.clone()).unwrap();
    config.set_from_opts(&matches, &opts, &args).unwrap();
    assert!(config.play_out_aftermath);
}
