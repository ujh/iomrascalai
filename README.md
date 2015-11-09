[![Build Status](https://travis-ci.org/ujh/iomrascalai.svg?branch=master)](https://travis-ci.org/ujh/iomrascalai)
[![Gitter chat](https://badges.gitter.im/ujh/iomrascalai.png)](https://gitter.im/ujh/iomrascalai)
[![Coverage Status](https://coveralls.io/repos/ujh/iomrascalai/badge.svg?branch=master)](https://coveralls.io/r/ujh/iomrascalai?branch=master)

Iomrascálaí
===========

Iomrascálaí
(Gaelic for wrestler, [see here for the pronunciation](https://raw.githubusercontent.com/ujh/iomrascalai/master/pronunciation.mp4))
is an AI for the game of Go/Weiqi/Baduk written in Rust. Please note
that we're using the Rust nightly build and not stable!

Development
===========

See the [issues](https://github.com/ujh/iomrascalai/issues) for
planned features and bugs and
[join the mailing list](https://groups.google.com/forum/#!forum/iomrascalai)
and [the chat](https://gitter.im/ujh/iomrascalai) for discussion.

Testing
=======

To play 10 games against GnuGo, install GoGui and run the
following command in the top level folder:

``` sh
BLACK="gnugo --mode gtp --chinese-rules --positional-superko --capture-all-dead --score aftermath --play-out-aftermath"
WHITE="./target/release/iomrascalai"
gogui-twogtp -auto -black "$BLACK" -white "$WHITE" -size 9 -alternate -time 5m -games 100 -sgffile gnugo-test
```

To run a game against GnuGo and view it in GoGui in real time use the following command (add `-auto` if a new game should automatically be started when a game is finished):

``` sh
BLACK="gnugo --mode gtp --chinese-rules --positional-superko --capture-all-dead --score aftermath --play-out-aftermath"
WHITE="./target/release/iomrascalai"
TWOGTP="gogui-twogtp -black \"$BLACK\" -white \"$WHITE\" -verbose -size 9"
gogui -computer-both -program "$TWOGTP" -size 9
```

Resources
=========

The following Go programs are available as source code and can serve
as inspiration:

* [HouseBot](https://github.com/ujh/HouseBot)
* [Pachi](http://pachi.or.cz/)
* [Orego](https://github.com/Orego/Orego)
* [libego](https://github.com/lukaszlew/libego)
* [Fuego](http://sourceforge.net/projects/fuego/)
* [oakfoam](http://oakfoam.com/)
* [GnuGo](https://www.gnu.org/software/gnugo/)
* [Brown](http://www.lysator.liu.se/~gunnar/gtp/brown-1.0.tar.gz)

License
=======

Iomrascálaí is licensed under GPL v3. See the
[LICENSE file](https://github.com/ujh/iomrascalai/blob/master/LICENSE)
for the complete license text.
