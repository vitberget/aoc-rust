# Advent of Code - Rust

Solving [Advent of Code](https://adventofcode.com/).

Experimenting with [Rust](https://www.rust-lang.org/), hopefully learning stuff :D. 

I most often solve AoC in [Clojure](https://clojure.org/) first. <https://github.com/vitberget/advent-of-code-clojure>

## Profiling, when available

Run with envirment flag `AOC_PROFILING` set to... anything.

```sh
AOC_PROFILING=1 cargo run --bin year2024_day11
```

## Input

1. Without arguments: take file content from `puzzles/year-YYYY-dayDD.txt`
2. Without argument `-`: read from stdin
3. Without other argument: take file content from file names as argument

## Aoc Utils

The `aoc-utils` binary can download puzzles and create template solutions.

```sh
cargo run --bin aoc-utils create 2025 1 
```

```sh
cargo run --bin aoc-utils download 2025 1
```
