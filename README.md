# Advent of Code 2023

My solutions for [2023's Advent of Code](https://adventofcode.com/2023).

## Goals

- Have fun & learn things
- Fast execution time (< 1 second for whole set of puzzles)
- Proper error propagation
  - Actually learn how to deal with Result types and propagate errors up instead panicking
  - Avoid crashes due to bad/unexpected inputs; use adversarial inputs if need be
- Avoiding external dependencies is a non-goal
  - But only include well-known and defacto-standard libraries for specific purposes (e.g. regex, itertools, Rayon, anyhow)

## Notes & Learnings

### Day 1: Trebuchet?!

A surprisingly tricky first puzzle. Part 2 was required some actual thinking.

I learned about reading from stdin. In particular, using `io::read_to_string(io::stdin())` to read the whole of it in one go.

### Day 2: Cube Conundrum

One of those puzzles that involve more parsing than actual calculations, but it was fun actually :). I learned some patterns to work with `Result` types and properly propagate them up the call-chain, and some other tricks, for example:

* Using `try_collect()` from `itertools` instead of `collect::<Result<_, _>>()` which is quite ugly.
* Using `Option::ok_or("error message")` for converting `Option`s to `Result`s and propagate them up with `?`. Super useful when parsing :)
* And another useful function for parsing is `String::strip_prefix()`; need to remember it for the future!

### Day 3: Gear Ratios

A grid puzzle with some tricky parsing problems, as the numbers on the grid span multiple cells and need to be consider as a single thing. I quite liked the use of `usize::wrapping_add_signed()` to be explicit about wrapping behavior and avoid having many back and forth `as isize/usize` conversions.
