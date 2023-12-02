# Advent of Code 2023

My solutions for [2023's Advent of Code](https://adventofcode.com/2023).

## Goals

- Have fun & learn things
- Fast execution time (< 1 second for whole set of puzzles)
- Proper error propagation
  - Actually learn how to deal with Result types and propagate errors up instead panicking
  - Avoid crashes due to bad/unexpected inputs; use adversarial inputs if need be
- Avoiding external dependencies is a non-goal
  - But only include well-known and defacto-standard libraries for specific purposes (e.g. regex, Rayon, anyhow)

## Notes & Learnings

### Day 1: Trebuchet?!

A surprisingly tricky first puzzle. Part 2 was required some actual thinking.

I learned about reading from stdin. In particular, using `io::read_to_string(io::stdin())` to read the whole of it in one go.
