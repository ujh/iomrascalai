#!/bin/bash

set -e
set -x

cargo build --release

java -jar kgsGtp.jar Imrscl.cfg
