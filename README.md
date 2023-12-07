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

## Running stuff

You need to have Rust installed (see "rustup").

To run a single daily solution use:

```bash
cargo run --bin day_02_cube_conundrum < inputs/02/input.txt
```

(You may type `day_02`<kbd>tab</kbd> to tab-complete the binary name 😉)

Or you can also pass it a sample file:

```bash
cargo run --bin day_02_cube_conundrum < inputs/02/sample.txt
```

To run all daily solutions with their input files run:

```
./run-all
```

And to check all sample files against their expected output and all input files against the answers on `answers.txt` run:

```
./check-all
```

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

### Day 4: Scratchcards

A simple and enjoyable Monday puzzle. Part 2 looked intimidating at first, but was relatively easy once given a little bit og thought :)

### Day 5: If You Give A Seed A Fertilizer

This one was a pretty straightforward part 1, but part 2 is too slow with the most naive approach of brute-forcing all the possible seed numbers. Luckily, Rust compiles to pretty efficient code, so even this brute-force solution was able to run in ~100 seconds on my machine :)

The `Iterator::tuples()` method from `itertools` was quite helpful in easily pairing numbers for part 2.

Update: after quite a bit of hacking, i managed to implement a non-brute-forcey solution based on interval math logic. This was a PITA; it took 2hs of hacking at vey questionable logic to to a working solution that yields the same answer. But it paid off: now the code runs in ~2ms instead of ~100s :D

Ignoring the pain, i think i've learned some important lessons regarding these kind of interval logic puzzles:

- Modeling abstract math operations like intersection and diff pays off *really* quickly. And the logic of these operations isn't even that involved!
- Using open-ended [start, end) ranges is better than start+length pairs. Way less fiddling with numbers and off-by-one errors.
