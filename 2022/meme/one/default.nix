{ pkgs }:
let
  inherit (pkgs) lib;
  inherit (lib) strings lists;
  sumAll = x: (builtins.foldl' (y: z: y + z) 0 x);
  elves = with strings;
    with builtins;
    map (x: builtins.map (y: toInt y) (splitString "\n" x))
    (splitString "\n\n" (readFile ../../inputs/day1.txt));
  max = with builtins;
    foldl' (a: b: lib.trivial.max a b) 0 (map (c: sumAll c) elves);
  topThree = sumAll
    (lists.take 3 (builtins.sort (c: d: c > d) (map (e: sumAll e) elves)));
  partOne = max;
  partTwo = topThree;
in "Part one: ${builtins.toString partOne}, Part two: ${
  builtins.toString partTwo
}"
