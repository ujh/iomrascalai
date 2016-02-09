/************************************************************************
 *                                                                      *
 * Copyright 2015 Urban Hafner, Igor Polyakov                           *
 * Copyright 2016 Urban Hafner                                          *
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

use ruleset::Ruleset;

use num_cpus;
use std::fs::File;
use std::io::prelude::*;
use std::io::stderr;
use std::process::exit;
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
    /// The number of plays before a leaf will be expanded.
    pub expand_after: usize,
    /// Configuration factor for the RAVE part of the node selection
    /// algorithm. There's no clear way to set this value. It's best
    /// to use parameter optimization to find the best value.
    pub rave_equiv: f32,
    /// A float between 0.0 and 1.0 that is the part of a win recorded
    /// in the tree nodes to signify the score of the playout.
    pub score_weight: f32,
}

impl TreeConfig {

    fn new(value: toml::Value, default: toml::Value) -> TreeConfig {
        let opts = value.as_table().unwrap().clone();
        let default_table = default.as_table().unwrap().clone();
        let mut table = toml::Table::new();
        table.extend(default_table);
        table.extend(opts);
        TreeConfig {
            expand_after: Self::as_integer(&table, "expand_after"),
            rave_equiv: Self::as_float(&table, "rave_equiv"),
            score_weight: Self::as_float(&table, "score_weight"),
        }
    }

}

impl FromToml for TreeConfig {
    fn name() -> Option<&'static str> { Some("tree") }
}

/// Holds all settings related to initializing the leaves of the
/// search tree with prior values for plays and wins.
#[derive(Debug, PartialEq)]
pub struct PriorsConfig {
    /// When calculating the number of wins and plays a node has (e.g.
    /// when calculating the win rate) this is the weight the priors
    /// are given. If it's 0 then they aren't taken into account at
    /// all. If they are 1 then it's equal to adding up both the
    /// actual plays and the priors plays. Larger values than 1 give
    /// increasingly higher weight to the prior values over the
    /// actually observed plays and wins.
    pub best_move_factor: f32,
    /// The prior for a move that captures more than one stone. It is
    /// an even prior, i.e. `capture_many` is added to both the prior
    /// plays and wins.
    pub capture_many: usize,
    /// Same as `capture_many` but for the case where a move captures
    /// a single stone.
    pub capture_one: usize,
    /// The prior to assign a move that plays close to the border.
    /// It's a negative prior (i.e. only prior plays are increased)
    /// when playing on the 1st and 2nd line and an even prior for
    /// moves on the third line. This is only applied if the area
    /// around the move of a Manhattan distance of three is empty.
    pub empty: usize,
    /// The number of prior plays to start with. This is useful to
    /// simplify the calculations as we can avoid 0 values.
    pub neutral_plays: usize,
    /// The number of prior wins to start with. This is normally 0.5
    /// of `neutral_plays` so that we start of with a win rate of 50%.
    pub neutral_wins: usize,
    /// The prior to assign when one of the 3x3 pattern matches. This
    /// is an even prior.
    pub patterns: usize,
    /// The prior to assign when the move puts one of our own groups
    /// in self atari. This is a negative prior (i.e. only prior plays
    /// are increased).
    pub self_atari: usize,
}

impl PriorsConfig {

    fn new(value: toml::Value, default: toml::Value) -> PriorsConfig {
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
    /// The percentage of the allocated time for the current move
    /// after which to check for early termination of the search.
    pub fastplay_budget: f32,
    /// Once `fastplay_budget` percent of the allocated time for a
    /// move have passed check if the best move has a win rate that is
    /// higher than this value. If so then stop the search and return
    /// this move.
    pub fastplay_threshold: f32,
    /// Minimum number of stones to use when calculating the budget
    /// for the next move.
    pub min_stones: usize,
}

impl TimeControlConfig {

    fn new(value: toml::Value, default: toml::Value) -> TimeControlConfig {
        let opts = value.as_table().unwrap().clone();
        let default_table = default.as_table().unwrap().clone();
        let mut table = toml::Table::new();
        table.extend(default_table);
        table.extend(opts);
        TimeControlConfig {
            c: Self::as_float(&table, "c"),
            fastplay_budget: Self::as_float(&table, "fastplay_budget"),
            fastplay_threshold: Self::as_float(&table, "fastplay_threshold"),
            min_stones: Self::as_integer(&table, "min_stones"),
        }
    }
}

impl FromToml for TimeControlConfig {
    fn name() -> Option<&'static str> { Some("time_control") }
}

/// Holds settings related to the playout policy
#[derive(Debug, PartialEq)]
pub struct PlayoutConfig {
    /// The probability of checking for atari moves (and playing one
    /// if there are any). Set to 1.0 to always check.
    pub atari_check: f32,
    /// The probability of using the ladder checker (which is
    /// expensive) during atari resolution. Set to 1.0 to always use
    /// it.
    pub ladder_check: f32,
    /// The number of most recently played moves to consider when
    /// selecting moves based on heuristics.
    pub last_moves_for_heuristics: usize,
    /// The probability of playing a move that was found by trying to
    /// match patterns on the current board. We don't want to always
    /// play those moves as this would reduce the random element of
    /// the playouts too much.
    pub pattern_probability: f32,
    /// ???
    pub play_in_middle_of_eye: f32,
}

impl PlayoutConfig {

    fn new(value: toml::Value, default: toml::Value) -> PlayoutConfig {
        let opts = value.as_table().unwrap().clone();
        let default_table = default.as_table().unwrap().clone();
        let mut table = toml::Table::new();
        table.extend(default_table);
        table.extend(opts);
        PlayoutConfig {
            atari_check: Self::as_float(&table, "atari_check"),
            ladder_check: Self::as_float(&table, "ladder_check"),
            last_moves_for_heuristics: Self::as_integer(&table, "last_moves_for_heuristics"),
            pattern_probability: Self::as_float(&table, "pattern_probability"),
            play_in_middle_of_eye: Self::as_float(&table, "play_in_middle_of_eye"),
        }
    }

}

impl FromToml for PlayoutConfig {
    fn name() -> Option<&'static str> { Some("playout") }
}

/// Hold settings related to estimating the score of a board
#[derive(Debug, PartialEq)]
pub struct ScoringConfig {
    /// Prior for the value of neutral owners (i.e. dame points). This
    /// increases the number of playouts necessary to generate an
    /// ownership value that's above the cutoff which increases the
    /// confidence.
    pub ownership_prior: usize,
    /// Value between 0.0 and 1.0 which is the cutoff above which a
    /// point is considered to be owned by a color.
    pub ownership_cutoff: f32,
}

impl ScoringConfig {

    fn new(value: toml::Value, default: toml::Value) -> ScoringConfig {
        let opts = value.as_table().unwrap().clone();
        let default_table = default.as_table().unwrap().clone();
        let mut table = toml::Table::new();
        table.extend(default_table);
        table.extend(opts);
        ScoringConfig {
            ownership_prior: Self::as_integer(&table, "ownership_prior"),
            ownership_cutoff: Self::as_float(&table, "ownership_cutoff"),
        }
    }

}

impl FromToml for ScoringConfig {
    fn name() -> Option<&'static str> { Some("scoring") }
}

/// This is the global configuration object. Is is passed around
/// (inside an `Arc`) most of the app and contains all possible
/// settings and variables that can be tuned. Everything in here can
/// be set in a configuration file in TOML format.
#[derive(Debug, PartialEq)]
pub struct Config {
    /// If `true` output GoGui live graphics commands on stderr so
    /// that you can see what the engine is "thinking" when playing or
    /// observing a game via GoGui
    pub gfx: bool,
    /// If `true` the various information (e.g. the number of
    /// simulations played) is printed to stderr while the engine is
    /// running.
    pub log: bool,
    /// Holds a configuration object that contains everything related
    /// to the playout policy.
    pub playout: PlayoutConfig,
    /// Holds a configuration object that contains everything related
    /// to setting prior values in the tree nodes.
    pub priors: PriorsConfig,
    /// The ruleset we're currently playing under (CGOS, chinese, etc.)
    pub ruleset: Ruleset,
    /// Holds a configuration object that contains everything related
    /// to estimating the score of a board
    pub scoring: ScoringConfig,
    /// The number of threads to use. The best results are achieved
    /// right now if this is the same number as the number of (logical)
    /// cores the computer has that the program runs on.
    pub threads: usize,
    /// Holds a configuration object that contains everything related
    /// to allocating the time to use for the next move, stopping the
    /// search early, etc.
    pub time_control: TimeControlConfig,
    /// Holds a configuration object athat contains everything related
    /// to the tree search (when to expand the leaves, RAVE
    /// configuration, etc.)
    pub tree: TreeConfig,
}

impl Config {

    /// Uses the TOML returned by `Config::toml()` and returns a
    /// `Config` object that encodes this data.
    pub fn default(log: bool, gfx: bool, ruleset: Ruleset) -> Config {
        Self::new(String::from(""), Self::toml(), log, gfx, ruleset)
    }

    /// Returns a string representation of the default configuration
    /// encoded as TOML.
    pub fn toml() -> String {
        String::from(include_str!("defaults.toml"))
    }

    /// Takes a `String` which should represent the path to a TOML
    /// encoded file. It reads that file and generates a `Config`
    /// object from the data. The file doesn't need to contain all
    /// possible fields of `Config` or the various structs it
    /// contains. What's missing is taken from `Config::toml()`.
    pub fn from_file(filename: String, log: bool, gfx: bool, ruleset: Ruleset) -> Config {
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        Self::new(contents, Self::toml(), log, gfx, ruleset)
    }

    fn new(toml_str: String, default_toml_str: String, log: bool, gfx: bool, ruleset: Ruleset) -> Config {
        let opts = toml::Parser::new(&toml_str).parse().unwrap();
        let default_table = toml::Parser::new(&default_toml_str).parse().unwrap();
        let threads = toml::Parser::new(&format!("threads = {}", num_cpus::get())).parse().unwrap();
        let mut table = toml::Table::new();
        table.extend(default_table.clone());
        table.extend(threads.clone());
        table.extend(opts.clone());
        Config {
            gfx: gfx,
            log: log,
            playout: PlayoutConfig::new(table["playout"].clone(), default_table["playout"].clone()),
            priors: PriorsConfig::new(table["priors"].clone(), default_table["priors"].clone()),
            ruleset: ruleset,
            scoring: ScoringConfig::new(table["scoring"].clone(), default_table["scoring"].clone()),
            threads: Self::as_integer(&table, "threads"),
            time_control: TimeControlConfig::new(table["time_control"].clone(), default_table["time_control"].clone()),
            tree: TreeConfig::new(table["tree"].clone(), default_table["tree"].clone()),
        }
    }

    /// If logging is turned on then the string passed will be printed
    /// to standard error. Otherwise it's silently discarded.
    pub fn log(&self, s: String) {
        if self.log {
            match stderr().write(format!("{}\n", s).as_bytes()) {
                Ok(_) => {},
                Err(x) => panic!("Unable to write to stderr: {}", x)
            }
        }
    }

    /// If GoGui live graphics support is turned on then this will
    /// output the commands on stderr. Otherwise they are silently
    /// discarded.
    pub fn gfx(&self, s: String) {
        if self.gfx {
            match stderr().write(format!("{}\n", s).as_bytes()) {
                Ok(_) => {},
                Err(x) => panic!("Unable to write to stderr: {}", x)
            }
        }
    }

}

impl FromToml for Config {
    fn name() -> Option<&'static str> { None }
}
