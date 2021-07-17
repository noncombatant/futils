#!/bin/sh

set -e

cargo build --release
cp target/release/futils ~/bin
strip ~/bin/futils
ln -f ~/bin/futils ~/bin/fields
ln -f ~/bin/futils ~/bin/filter
ln -f ~/bin/futils ~/bin/map
ln -f ~/bin/futils ~/bin/records
ln -f ~/bin/futils ~/bin/reduce
ln -f ~/bin/futils ~/bin/sum
ln -f ~/bin/futils ~/bin/zip
