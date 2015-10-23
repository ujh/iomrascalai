set -e
set -x

git co with-pattern-prior
./bin/benchmark with-pattern-prior
