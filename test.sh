#!/usr/bin/env bash

set -o errexit
set -o errtrace
set -o nounset
set -o pipefail
shopt -s extdebug

f="target/debug/futils"

echo "./test/goat" | diff -d - <(${f} files -m goat)
echo "./test/Goats
./test/goat" | diff -d - <(${f} files -m '(?i)goat')
echo "./test/Goats" | diff -d - <(${f} files -m '(?i)goats')
echo "./test/lurp/norp/yibb" | diff -d - <(${f} files -m p/y)

echo "yeah
whee" | diff -d - <(${f} fields -f 1 test/columns.txt)
echo "1	yeah	hey
2	whee	ouch" | diff -d - <(${f} fields -f 1 -f 3 -n test/columns.txt)
