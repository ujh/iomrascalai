#!/bin/sh
# Script to install prerequisites on a Linux box (like an Amazon EC2 instance).

set -e
set -x

sudo apt-get -y update
sudo apt-get -y upgrade
sudo apt-get -y install htop wget emacs24 default-jre binutils gcc git

# TODO: Install a custom GnuGo that doesn't crash when playing certain ladders
sudo apt-get -y install gnugo

# Install the lastest nightly Rust
sudo curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly

# Install GoGui
if [ ! -f gogui-1.4.9.zip ]; then
  wget http://downloads.sourceforge.net/project/gogui/gogui/1.4.9/gogui-1.4.9.zip
fi
if [ ! -d gogui-1.4.9 ]; then
  unzip gogui-1.4.9.zip
  echo "export PATH=~/gogui-1.4.9/bin:\$PATH" >> .bashrc
  source .bashrc
fi

# Get the latest source for Iomrascálaí
if [ ! -d iomrascalai ]; then
  git clone https://github.com/ujh/iomrascalai.git
fi
cd iomrascalai
git co master
git pull
cd ..
