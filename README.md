[![Build Status](https://travis-ci.org/ujh/iomrascalai.svg?branch=master)](https://travis-ci.org/ujh/iomrascalai)
[![Gitter chat](https://badges.gitter.im/ujh/iomrascalai.png)](https://gitter.im/ujh/iomrascalai)
[![Coverage Status](https://coveralls.io/repos/ujh/iomrascalai/badge.svg?branch=master)](https://coveralls.io/r/ujh/iomrascalai?branch=master)

Iomrascálaí
===========

Iomrascálaí (Gaelic for wrestler, [see here for the pronunciation](https://raw.githubusercontent.com/ujh/iomrascalai/master/pronunciation.mp4)) is an AI for the game of Go/Weiqi/Baduk written in [Rust](https://www.rust-lang.org).

Installation
------------

Iomrascálaí requires the latest unstable (also called nightly) Rust compiler as well as the latest Cargo. Both are generally installed when you download the installers from the [Rust homepage](https://www.rust-lang.org). You will also need a graphical interface for playing games against the AI. We suggest downloading GoGui (**LINK**).

Playing games
-------------

Once you've installed the nightly Rust compiler, Cargo, and [GoGui](http://gogui.sourceforge.net/) you can use the following shell scripts to compile the program and play games:

* `bin/play` will compile the program and start a game on a 9x9 board with a time limit of 5 minutes (sudden death) in [GoGui](http://gogui.sourceforge.net/). By default it will assign black to Iomrascálaí. The defaults can be changed easily by editing some constants in the script.
* `bin/play-gnugo` will compile the program and start a game on 9x9 with a time limit of 5 minutes (sudden death) against GnuGo. It will assign black to [GnuGo](https://www.gnu.org/software/gnugo/) and the game can be observed in [GoGui](http://gogui.sourceforge.net/). Again, the defaults (board size, time limits, etc.) can be changed by editing the script.
* `bin/play-self` will compile the program and start a game on 9x9 with a time limit of 5 minutes (sudden death) between two copies of Iomrascálaí. Just like with the other scripts the game can be observed in [GoGui](http://gogui.sourceforge.net/) and the parameters can be adjusted by editing the script.

Program parameters
------------------

Many parameters of Iomrascálaí can be changed and those changes directly affect the program strength.

You set the parameters by supplying a TOML (**LINK**) formatted configuration file when starting the program with either the `-c` or `--config` command line flag. A great way to get started is to capture the output of `-d` (or `--dump`) into a file and edit the variables. This is the default configuration and lists all possible variables with their default values. See the [api documentation](http://bettong.net/iomrascalai/api/iomrascalai) for details on what these variables do. Just use the search on top and enter the name of the configuration variable. It should take you to the page that lists the struct that defines it (e.g. `PriorsConfig` for a variable in the `[priors]` block in the config file).

Development
===========

See the [issues](https://github.com/ujh/iomrascalai/issues) for
planned features and bugs and
[join the mailing list](https://groups.google.com/forum/#!forum/iomrascalai)
and [the chat](https://gitter.im/ujh/iomrascalai) for discussion.

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
