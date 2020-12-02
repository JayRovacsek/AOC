#!/bin/sh

if [ "$#" -eq  "0" ]
then
    echo "No day argument supplied :("
    echo "Please supply day as string representation of value; e.g. 2 = \"two\""
else
    cargo bench > "bench_results/day_$1"
fi