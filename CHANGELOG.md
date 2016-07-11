## 0.3.2 [☰](https://github.com/ujh/iomrascalai/compare/0.3.1...0.3.2)

* Fixes a bug where the bot passed too early and lost won games because of it.
  #261
* Resign lost games earlier. This speeds up the benchmark and is also nicer for
  humans. #264
* Speed up the calculations of the priors by moving them into the worker
  threads. #275
* Implement the `reg_genmove` GTP command. #289
* Implement the `kgs-genmove_cleanup` GTP command. #290
* Improve the scoring accuracy when the game is over. #292
* Fix bug where some rotations of the small patterns were missing. #299
* Implemented the `imrscl-uct_gfx` custom GTP command. A GoGui analyze
  compatible command to show the number of playouts that went through each child
  node of the root (i.e. each point on the board) #293

### Performance

*The benchmarks were run on a c4.8xlarge EC2 instance with 8 worker threads.*

After running 500 games on 9x9 with komi 6.5 and a time limit of 2
minutes (sudden death) the win rate against GnuGo 3.8 level 0 was
**76.2% (± 3.71 at 95%, ± 3.79 at 99%)** with the default
configuration. It agreed on the final score with GnuGo in **74.09% (± 4.35 at
95%, ± 4.44 at 99%)** of games.

After running 1000 games on 13x13 with komi 6.5 and a time limit of 10
minutes (sudden death) the win rate against GnuGo 3.8 level 0 was
**42.8% (± 3.05 at 95%, ± 3.11 at 99%)** with the default
configuration. It agreed on the final score with GnuGo in **59.78% (± 4.53 at
95%, ± 4.63 at 99%)** of games.


## 0.3.1 [☰](https://github.com/ujh/iomrascalai/compare/0.3.0...0.3.1)

* Implemented `gogui-analyze_commands` and `imrscl-safe_territory`
  which shows the intersections on the board that the bot thinks are
  safe (i.e. definitely belong to one color).
* Stop playing when all intersections are decided (i.e. the bot thinks
  they belong to a certain color).
* Implement `final_status_list`
* Implement Chinese scoring
* Return to command line arguments for turning on/off logging, GoGui
  effects, and ruleset configuration.

### Performance

*Note that we now use Chinese rules instead of CGOS/Tromp Taylor rules
 which may change the results.*

After running 500 games on 9x9 with komi 6.5 and a time limit of 5
minutes (sudden death) the win rate against GnuGo 3.8 level 0 was
**77.4% (± 3.65 at 95%, ± 3.72 at 99%)** with the default
configuration.

After running 500 games on 13x13 with komi 6.5 and a time limit of 10
minutes (sudden death) the win rate against GnuGo 3.8 level 0 was
**35.2% (± 4.16 at 95%, ± 4.25 at 99%)** with the default
configuration.


## 0.3.0 [☰](https://github.com/ujh/iomrascalai/compare/0.2.4...0.3.0)

* Added 3x3 pattern support. These are used as priors in the tree and
  during the playouts.
* Added the RAVE heuristic to the search.
* Terminate search early if the best move is ahead by a lot.
* Changed the way to change the configuration of the engine.
  Everything is now stored inside a
  [TOML](https://github.com/toml-lang/toml) file instead of having
  command line switches for everything.
* Measure the playouts per second per thread and display it when
  logging is turned on.
* Scripts to run [CLOP](http://www.remi-coulom.fr/CLOP/) parameter
  optimization.
* Removed all weak engines (random, AMAF, MC).
* Clarified the licensing (scripts are MIT licensed).
* Added a code of conduct.

### Performance

*Note that the measurement of the error margins have changed. We now
 calculate proper 95% and 99% confidence intervals whereas previously
 we only used the ~68% standard deviation.*

After running 500 games on 9x9 with komi 6.5 and a time limit of 5
minutes (sudden death) the win rate against GnuGo 3.8 level 0 was
**85.40% (± 3.08 at 95%, ± 3.14 at 99%)** with the default
configuration.

After running 500 games on 13x13 with komi 6.5 and a time limit of 10
minutes (sudden death) the win rate against GnuGo 3.8 level 0 was
**36.6% (± 4.2 at 95%, ± 4.29 at 99%)** with the default
configuration.


## 0.2.4 [☰](https://github.com/ujh/iomrascalai/compare/0.2.3...0.2.4)

This release doesn't contain any new features. But as the last release
happened quite a long time ago and there were some (small) internal
changes a new release seems to be in order. Also the Rust compiler
changed a lot in between and changed the performance.

### Performance

After running 200 games on 9x9 with komi 6.5 and a time limit of 5
minutes (sudden death) the win rate against GnuGo 3.8 level 0 was
**65.5% ± 3.4** for the default engine with 8 threads.

After running 200 games on 13x13 with komi 6.5 and a time limit of 10
minutes (sudden death) the win rate against GnuGo 3.8 level 0 was
**10% ± 2.1** for the default engine with 8 threads.

## 0.2.3 [☰](https://github.com/ujh/iomrascalai/compare/0.2.2...0.2.3)

* Playout the aftermath by default on CGOS
* Play in the middle of big eyes during playouts
* Resign lost games faster (to save time)

### Performance

After running 200 games on 9x9 with komi 6.5 and a time limit of 5
minutes (sudden death) the win rate against GnuGo 3.8 level 0 was **75.5%
± 3%** for the default engine with 8 threads.

After running 200 games on 13x13 with komi 6.5 and a time limit of 10
minutes (sudden death) the win rate against GnuGo 3.8 level 0 was **6%
± 1.7%** for the default engine with 8 threads.

## 0.2.2 [☰](https://github.com/ujh/iomrascalai/compare/0.2.1...0.2.2)

* A two liberty solver (can prove if a group with up to two stones is
  dead)
* A set of priors for the UCT tree
  * One that prioritizes killing opponent groups
  * One that discourages putting one's own groups into danger
  * One to disourage plays on the first two lines if no other stones
    are close by

### Performance

After running 200 games on 9x9 with komi 6.5 and a time limit of 5
minutes (sudden death) the win rate against GnuGo 3.8 level 0 was **70%
± 3.2%** for the default engine with 8 threads.

After running 200 games on 13x13 with komi 6.5 and a time limit of 10
minutes (sudden death) the win rate against GnuGo 3.8 level 0 was **11%
± 2.2%** for the default engine with 8 threads.


## 0.2.1 [☰](https://github.com/ujh/iomrascalai/compare/0.2.0...0.2.1)

* New UCT policy UCB1tuned
* Reuse the subtree from the previous genmove
* Seki detection in the self atari code in the playouts

### Performance

After running 100 games on 9x9 with komi 6.5 and a time limit of 5
minutes (sudden death) the win rate against GnuGo 3.8 level 0 was **35%
± 4.8%** for the default engine with 8 threads.

## 0.2.0 [☰](https://github.com/ujh/iomrascalai/compare/0.1.7...0.2.0)

* UCT engine
* A new playout policy that doesn't put large strings into self atari

## 0.1.7 [☰](https://github.com/ujh/iomrascalai/compare/0.1.6...0.1.7)

* Improvements of the playout speeds
* The playout type can now be selected from the command line (use the
  -p command line option)

## 0.1.6 [☰](https://github.com/ujh/iomrascalai/compare/0.1.5...0.1.6)

* The engines can now utilize multiple threads (use the -t command
  line switch)
* Debug output to stderr is now only generated when running with -l
* The code coverage is now measured and reported through a badge in
  the README

## 0.1.5 [☰](https://github.com/ujh/iomrascalai/compare/0.1.4...0.1.5)

### Changes

* Fix bug in AMAF code that made it much weaker than it should be. The
  first move of each playout wasn't taken into account during the move
  selection ([#102](https://github.com/ujh/iomrascalai/pull/102))
* Implemented the loadsgf GTP command ([#104](https://github.com/ujh/iomrascalai/pull/104))

### Performance

After running 100 games on 9x9 with komi 6.5 and a time limit of 5
minutes (sudden death) the win rates are as follows:

* 3% ± 1.7% of the MC engine against GnuGo 3.8 (level 0)
* 3% ± 1.7% of the AMAF engine against GnuGo 3.8 (level 0)
* 92% ± 2.7% of the MC engine against AmiGoGtp 1.8
* 92% ± 2.7% of the AMAF engine against AmiGoGtp 1.8
* 41% ± 4.9% of the MC engine against the MC engine of 0.1.4
* 52% ± 5% of the AMAF engine against the AMAF engine of 0.1.4
* 70% ± 4.6% of the current AMAF engine against the current MC engine

## 0.1.4 [☰](https://github.com/ujh/iomrascalai/compare/0.1.3...0.1.4)

### Changes

* Make AMAF the default engine. This was supposed to be the default
  for 0.1.3, but didn't happen so the win rates recorded for 0.1.3 are
  useless!
* Refactored the engine code, to but the shared code of the MC and the
  AMAF into a trait.

### Performance

After running 100 games on 9x9 with komi 6.5 and a time limit of 5
minutes (sudden death) the win rates are as follows:

* 10% ± 3% against Iomrascálaí 0.1.3
* 2% ± 1.4% against GnuGo 3.8 (Level 0)
* 70% ± 4.6% win rate of the AMAF engine against the MC engine

## 0.1.3 [☰](https://github.com/ujh/iomrascalai/compare/0.1.2...0.1.3)

### Changes

* A new engine implementing the AMAF (all moves as first) pattern.
  Almost the same as standard Monte-Carlo, but recording wins and
  losses for all moves of the simulated color.
* Don't pass even when the win rate calculated through the simulations
  is 100%. Doing this resulted in losses against a random player.
* Scripts to play a game in GoGui against GnuGo and Brown.
* Refactored the time keeping code. It has now moved out of the
  engines and lives in a separate struct.

### Performance

After running 100 games on 9x9 with komi 6.5 and a time limit of 5
minutes (sudden death) the win rates are as follows:

* 49% ± 5% against Iomrascálaí 0.1.2
* 5% ± 2.2% against GnuGo 3.8 (Level 0)

## 0.1.2 [☰](https://github.com/ujh/iomrascalai/compare/0.1.1...0.1.2)

### Changes

* A small change to the playout policy: Only play the pass move if no
  other moves (that are not plays into single point eyes) are
  possible.
* Add GnuGo as the referee in the benchmark, so that we always get a
  score, even if the bots disagree.
* Enhance the benchmark script to also play against the previous
  version of the bot.

### Performance

After running 100 games on 9x9 with komi 6.5 and a time limit of 5
minutes (sudden death) the win rates are as follows:

* 1% ± 1% against Iomrascálaí 0.1.1
* 7% ± 2.6% against GnuGo 3.8 (Level 0)
