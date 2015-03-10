## 0.1.4 [☰](https://github.com/ujh/iomrascalai/compare/0.1.3...master)

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
