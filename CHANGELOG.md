## 0.1.2 [☰](https://github.com/ujh/iomrascalai/compare/0.1.1...master) (unreleased)

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

* XY% ± AB% against Iomrascálaí 0.1.1
* XY% ± AB% against GnuGo 3.8 (Level 0)
