#!/bin/sh
# Script to install prerequisites on a Linux (Ubuntu 14.04) box (like an Amazon EC2 instance).

set -e
set -x

sudo apt-get -y update
sudo apt-get -y upgrade
sudo apt-get -y install htop wget emacs24 binutils gcc git ruby unzip tmux

# Oracle Java is required by the kgsGtp client
sudo echo "deb http://ppa.launchpad.net/webupd8team/java/ubuntu xenial main" | sudo tee /etc/apt/sources.list.d/webupd8team-java.list
sudo echo "deb-src http://ppa.launchpad.net/webupd8team/java/ubuntu xenial main" | sudo tee -a /etc/apt/sources.list.d/webupd8team-java.list
sudo apt-key adv --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys EEA14886
sudo apt-get update
sudo apt-get -y install oracle-java8-installer

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
fi

# Get the latest source for Iomrascálaí
if [ ! -d iomrascalai ]; then
  git clone https://github.com/ujh/iomrascalai.git
fi
cd iomrascalai
git checkout master
git pull
cd ..
