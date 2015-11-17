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

#![deny(missing_docs)]

use ruleset::CGOS;
use ruleset::Ruleset;

use std::fs::File;
use std::io::prelude::*;
use std::io::stderr;
use std::process::exit;
use std::str::FromStr;
use toml;

trait FromToml {

    fn as_float(table: &toml::Table, field: &'static str) -> f32 {
        let value = &table[field];
        match value.type_str() {
            "integer" => value.as_integer().unwrap() as f32,
            "float" => value.as_float().unwrap() as f32,
            _ => Self::fail(field, value, "float")
        }
    }

    fn as_integer(table: &toml::Table, field: &'static str) -> usize {
        let value = &table[field];
        match value.type_str() {
            "integer" => value.as_integer().unwrap() as usize,
            "float" => value.as_float().unwrap() as usize,
            _ => Self::fail(field, value, "integer")
        }
    }

    fn as_bool(table: &toml::Table, field: &'static str) -> bool {
        let value = &table[field];
        match value.as_bool() {
            Some(v) => v,
            None => Self::fail(field, value, "boolean")
        }
    }

    fn fail(field: &'static str, value: &toml::Value, expected: &'static str) -> ! {
        let long_name = match Self::name() {
            Some(name) => format!("{}.{}", name, field),
            None => format!("{}", field)
        };
        println!("Expected {} for {:?} but found {}", expected, long_name, value.type_str());
        exit(1)
    }


    fn name() -> Option<&'static str>;

}

/// Contains all settings that are related to the search tree.
#[derive(Debug, PartialEq)]
pub struct TreeConfig {
    /// If the win rate of the best move at the end of the allocated
    /// search time for the next move is lower than this value then we
    /// resign. This is a hack until we implement estimating the score
    /// based on the playouts.
    pub end_of_game_cutoff: f32,
    /// The number of plays before a leaf will be expanded.
    pub expand_after: usize,
    /// Configuration factor for the RAVE part of the node selection
    /// algorithm. There's no clear way to set this value. It's best
    /// to use parameter optimization to find the best value.
    pub rave_equiv: f32,
    /// If this is `true` then we reuse the subtree from the last move
    /// we searched so that we don't have to start with an empty tree.
    pub reuse_subtree: bool,
}

impl TreeConfig {

    pub fn new(value: toml::Value, default: toml::Value) -> TreeConfig {
        let opts = value.as_table().unwrap().clone();
        let default_table = default.as_table().unwrap().clone();
        let mut table = toml::Table::new();
        table.extend(default_table);
        table.extend(opts);
        TreeConfig {
            end_of_game_cutoff: Self::as_float(&table, "end_of_game_cutoff"),
            expand_after: Self::as_integer(&table, "expand_after"),
            rave_equiv: Self::as_float(&table, "rave_equiv"),
            reuse_subtree: Self::as_bool(&table, "reuse_subtree"),
        }
    }

}

impl FromToml for TreeConfig {
    fn name() -> Option<&'static str> { Some("tree") }
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

    pub fn new(value: toml::Value, default: toml::Value) -> PriorsConfig {
        let opts = value.as_table().unwrap().clone();
        let default_table = default.as_table().unwrap().clone();
        let mut table = toml::Table::new();
        table.extend(default_table);
        table.extend(opts);
        PriorsConfig {
            best_move_factor: Self::as_float(&table, "best_move_factor"),
            capture_many: Self::as_integer(&table, "capture_many"),
            capture_one: Self::as_integer(&table, "capture_one"),
            empty: Self::as_integer(&table, "empty"),
            neutral_plays: Self::as_integer(&table, "neutral_plays"),
            neutral_wins: Self::as_integer(&table, "neutral_wins"),
            patterns: Self::as_integer(&table, "patterns"),
            self_atari: Self::as_integer(&table, "self_atari"),
            use_empty: Self::as_bool(&table, "use_empty"),
            use_patterns: Self::as_bool(&table, "use_patterns"),
        }
    }

}

impl FromToml for PriorsConfig {
    fn name() -> Option<&'static str> { Some("priors") }
}

/// Holds all settings related to time control.
#[derive(Debug, PartialEq)]
pub struct TimeControlConfig {
    /// Scaling factor for allocating the time for the next move. We
    /// devide the remaining time by `c * <EMPTY INTERSECTION COUNT>`.
    /// To make sure we never run out of time we set the empty
    /// intersection count to 30 if there are less than 30 empty
    /// intersections on the board.
    pub c: f32,
    /// Once 20% of the allocated time for a move have passed check if
    /// the best move has a win rate that is higher than this value.
    /// If so then stop the search and return this move.
    pub fastplay20_thres: f32,
    /// Once 5% of the allocated time for a move have passed check if
    /// the best move has a win rate that is higher than this value.
    /// If so then stop the search and return this move.
    pub fastplay5_thres: f32,
}

impl TimeControlConfig {

    pub fn new(value: toml::Value, default: toml::Value) -> TimeControlConfig {
        let opts = value.as_table().unwrap().clone();
        let default_table = default.as_table().unwrap().clone();
        let mut table = toml::Table::new();
        table.extend(default_table);
        table.extend(opts);
        TimeControlConfig {
            c: Self::as_float(&table, "c"),
            fastplay20_thres: Self::as_float(&table, "fastplay20_thres"),
            fastplay5_thres: Self::as_float(&table, "fastplay5_thres"),
        }
    }
}

impl FromToml for TimeControlConfig {
    fn name() -> Option<&'static str> { Some("time_control") }
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

    pub fn new(value: toml::Value, default: toml::Value) -> PlayoutConfig {
        let opts = value.as_table().unwrap().clone();
        let default_table = default.as_table().unwrap().clone();
        let mut table = toml::Table::new();
        table.extend(default_table);
        table.extend(opts);
        PlayoutConfig {
            atari_check: Self::as_bool(&table, "atari_check"),
            ladder_check: Self::as_bool(&table, "ladder_check"),
            last_moves_for_heuristics: Self::as_integer(&table, "last_moves_for_heuristics"),
            no_self_atari_cutoff: Self::as_integer(&table, "no_self_atari_cutoff"),
            pattern_probability: Self::as_float(&table, "pattern_probability"),
            play_in_middle_of_eye: Self::as_bool(&table, "play_in_middle_of_eye"),
            use_patterns: Self::as_bool(&table, "use_patterns"),
        }
    }

}

impl FromToml for PlayoutConfig {
    fn name() -> Option<&'static str> { Some("playout") }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub log: bool,
    pub play_out_aftermath: bool,
    pub playout: PlayoutConfig,
    pub priors: PriorsConfig,
    pub ruleset: Ruleset,
    pub threads: usize,
    pub time_control: TimeControlConfig,
    pub tree: TreeConfig,
}

impl Config {

    pub fn default() -> Config {
        Self::new(String::from(""), Self::toml())
    }

    pub fn toml() -> String {
        String::from(include_str!("defaults.toml"))
    }

    pub fn from_file(filename: String) -> Config {
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        Self::new(contents, Self::toml())
    }

    fn new(toml_str: String, default_toml_str: String) -> Config {
        let opts = toml::Parser::new(&toml_str).parse().unwrap();
        let default_table = toml::Parser::new(&default_toml_str).parse().unwrap();
        let mut table = toml::Table::new();
        table.extend(default_table.clone());
        table.extend(opts.clone());
        let mut c = Config {
            log: Self::as_bool(&table, "log"),
            play_out_aftermath: Self::as_bool(&table, "play_out_aftermath"),
            playout: PlayoutConfig::new(table["playout"].clone(), default_table["playout"].clone()),
            priors: PriorsConfig::new(table["priors"].clone(), default_table["priors"].clone()),
            ruleset: Ruleset::from_str(table["ruleset"].as_str().unwrap()).unwrap(),
            threads: Self::as_integer(&table, "threads"),
            time_control: TimeControlConfig::new(table["time_control"].clone(), default_table["time_control"].clone()),
            tree: TreeConfig::new(table["tree"].clone(), default_table["tree"].clone()),
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

    fn set_ruleset_dependent_defaults(&mut self) {
        match self.ruleset {
            CGOS => {
                self.play_out_aftermath = true;
            }
            _ => {}
        }
    }

}

impl FromToml for Config {
    fn name() -> Option<&'static str> { None }
}
