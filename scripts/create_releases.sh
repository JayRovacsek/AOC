#!/bin/sh

echo $0
./scripts/release_script.sh $0 'JayRovacsek/AOC-2019' "v$1" -- ./release/*.tar.gz
