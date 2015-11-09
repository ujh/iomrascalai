/************************************************************************
 *                                                                      *
 * Copyright 2015 Urban Hafner, Igor Polyakov                           *
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

use ruleset::CGOS;
use ruleset::Ruleset;

use std::io::prelude::*;
use std::fs::File;
use std::io::stderr;
use std::str::FromStr;
use toml;

mod test;

#[derive(Debug, PartialEq)]
pub struct TreeConfig {
    pub end_of_game_cutoff: f32,
    pub expand_after: usize,
    pub fastplay20_thres: f32,
    pub fastplay5_thres: f32,
    pub rave_equiv: f32,
    pub reuse_subtree: bool,
}

impl TreeConfig {

    pub fn new(value: &toml::Value) -> TreeConfig {
        let table = value.as_table().unwrap();
        TreeConfig {
            end_of_game_cutoff: table["end_of_game_cutoff"].as_float().unwrap() as f32,
            expand_after: table["expand_after"].as_integer().unwrap() as usize,
            fastplay20_thres: table["fastplay20_thres"].as_float().unwrap() as f32,
            fastplay5_thres: table["fastplay5_thres"].as_float().unwrap() as f32,
            rave_equiv: table["rave_equiv"].as_float().unwrap() as f32,
            reuse_subtree: table["reuse_subtree"].as_bool().unwrap(),
        }
    }

}

#[derive(Debug, PartialEq)]
pub struct PriorsConfig {
    pub best_move_factor: f32,
    pub capture_many: usize,
    pub capture_one: usize,
    pub empty: usize,
    pub neutral_plays: usize,
    pub neutral_wins: usize,
    pub patterns: usize,
    pub self_atari: usize,
    pub use_empty: bool,
    pub use_patterns: bool,
}

impl PriorsConfig {

    pub fn new(value: &toml::Value) -> PriorsConfig {
        let table = value.as_table().unwrap();
        PriorsConfig {
            best_move_factor: table["best_move_factor"].as_float().unwrap() as f32,
            capture_many: table["capture_many"].as_integer().unwrap() as usize,
            capture_one: table["capture_one"].as_integer().unwrap() as usize,
            empty: table["empty"].as_integer().unwrap() as usize,
            neutral_plays: table["neutral_plays"].as_integer().unwrap() as usize,
            neutral_wins: table["neutral_wins"].as_integer().unwrap() as usize,
            patterns: table["patterns"].as_integer().unwrap() as usize,
            self_atari: table["self_atari"].as_integer().unwrap() as usize,
            use_empty: table["use_empty"].as_bool().unwrap(),
            use_patterns: table["use_patterns"].as_bool().unwrap(),
        }
    }

}

#[derive(Debug, PartialEq)]
pub struct TimerConfig {
    pub c: f32,
}

impl TimerConfig {

    pub fn new(value: &toml::Value) -> TimerConfig {
        let table = value.as_table().unwrap();
        TimerConfig {
            c: table["c"].as_float().unwrap() as f32
        }
    }

}

#[derive(Debug, PartialEq)]
pub struct PlayoutConfig {
    pub atari_check: bool,
    pub ladder_check: bool,
    pub last_moves_for_heuristics: usize,
    pub no_self_atari_cutoff: usize,
    pub pattern_probability: f32,
    pub play_in_middle_of_eye: bool,
    pub use_patterns: bool,
}

impl PlayoutConfig {

    pub fn new(value: &toml::Value) -> PlayoutConfig {
        let table = value.as_table().unwrap();
        PlayoutConfig {
            atari_check: table["atari_check"].as_bool().unwrap(),
            ladder_check: table["ladder_check"].as_bool().unwrap(),
            last_moves_for_heuristics: table["last_moves_for_heuristics"].as_integer().unwrap() as usize,
            no_self_atari_cutoff: table["no_self_atari_cutoff"].as_integer().unwrap() as usize,
            pattern_probability: table["pattern_probability"].as_float().unwrap() as f32,
            play_in_middle_of_eye: table["play_in_middle_of_eye"].as_bool().unwrap(),
            use_patterns: table["use_patterns"].as_bool().unwrap(),
        }
    }

}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub log: bool,
    pub play_out_aftermath: bool,
    pub playout: PlayoutConfig,
    pub priors: PriorsConfig,
    pub ruleset: Ruleset,
    pub threads: usize,
    pub timer: TimerConfig,
    pub tree: TreeConfig,
}

impl Config {

    pub fn default() -> Config {
        Self::new(String::from(Self::toml()))
    }

    pub fn toml() -> &'static str {
        include_str!("defaults.toml")
    }

    pub fn from_file(filename: String) -> Config {
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        Self::new(contents)
    }

    fn new(toml_str: String) -> Config {
        let table = toml::Parser::new(&toml_str).parse().unwrap();
        let mut c = Config {
            log: table["log"].as_bool().unwrap(),
            play_out_aftermath: table["play_out_aftermath"].as_bool().unwrap(),
            playout: PlayoutConfig::new(&table["playout"]),
            priors: PriorsConfig::new(&table["priors"]),
            ruleset: Ruleset::from_str(table["ruleset"].as_str().unwrap()).unwrap(),
            threads: table["threads"].as_integer().unwrap() as usize,
            timer: TimerConfig::new(&table["timer"]),
            tree: TreeConfig::new(&table["tree"]),
        };
        c.set_ruleset_dependent_defaults();
        c
    }

    pub fn log(&self, s: String) {
        if self.log {
            match stderr().write(format!("{}\n", s).as_bytes()) {
                Ok(_) => {},
                Err(x) => panic!("Unable to write to stderr: {}", x)
            }
        }
    }

    pub fn check(&self) -> Result<(), String> {
        if self.playout.ladder_check && !self.playout.atari_check {
            let s = String::from("'--use-ladder-check-in-playouts true' requires '--use-atari-check-in-playouts true'");
            Err(s)
        } else {
            Ok(())
        }
    }

    fn set_ruleset_dependent_defaults(&mut self) {
        match self.ruleset {
            CGOS => {
                self.play_out_aftermath = true;
            }
            _ => {}
        }
    }

}
