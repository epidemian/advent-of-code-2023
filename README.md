# Advent of Code 2023

My solutions for [2023's Advent of Code](https://adventofcode.com/2023).

Daily solutions' code in in the [`src/bin`](src/bin) directory.

I recorded my solves on [this YouTube playlist](https://www.youtube.com/playlist?list=PL3kymB6hDjyVK6P1j7TgqxmOtjxXmzbFc).

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

### Day 6: Wait For It

A very simple puzzle that could be solved by brute force and still run fast enough. My input required to check ~50M numbers, just takes like ~5ms on my machine, so no need to find a better algorithm. Thanks again, Rust! :)

### Day 7: Camel Cards

Nice domain-modeling puzzle. Part 2 was a neat twist to rethink some assumptions.

Learned the trick of using `.zip(1..)` instead of `.enumerate()` to enumerate things starting from 1.

Once again, i discover that implementing a custom `Ord` is not the most trivial thing. And learned a couple of lessons regarding ordering:

- Prefer using tuples of things as order keys, instead of custom `self.foo.cmp(&other.foo).then_with(|| ...)` chains. Tuples or arrays already implement lexicographic order.
- If possible, prefer deriving all ordering logic using `#[derive(Ord, PartialOrd)]`. Enums can be trivially ordered, and structs get lexicographically ordered by their fields.


### Day 8: Haunted Wasteland

Trivial part 1 and quite tricky part 2. It didn't take me so long, but i stumbled upon the answer by chance really: it seemed that it was *somewhere* along the lines of counting loops through the node network and then doing some least common multiple of the times it took each ghost to reach an end. But i didn't expect to be just that! If you watch [the recording](https://youtu.be/UdNuH2UJzNs) for that day, you can see me getting both answers in ~1 hour and then spending another whole hour trying to figure out *why* the second answer was right.

### Day 9: Mirage Maintenance

Super nice and simple "mathematical" puzzle. A recursive solution worked wonderfully :chef-kiss:

Although, i learned that Rust's `Result` propagation does not provide an easy way of printing the stack trace by default. Not even the line location of the original error :(

There are some proposals to make error handling more ergonomic. See: https://github.com/rust-lang/rust/issues/53487

### Day 10: Pipe Maze

Super challenging puzzle. Part 1 was relatively straightforward, but part 2... damn.

I couldn't come up with a clever and nice solution for part 2, so what i did was to "expand" the original grid threefold, converting each pipe tile into a 3x3 drawing of that pipe on the expanded grid. For example, the 4 tiles `L-7|` would get expanded to:


```
.#........#.
.#######..#.
.......#..#.
```

This allowed two pipes that seemed to be originally "touching", like the `7|` above, to have some actual space between them in the expanded grid.

Then i flood-filled this expanded grid starting from the outside, so only the space inside the giant pipe loop does not get filled. And then it's easy to know which original tiles are inside the giant loop.

The "advantage" of this approach is that parts 1 and 2 are very independent. None of the pipe-walking logic from part 1 is relevant for part 2. And none of the part 2 flood-filling logic is relevant for part 1 either.

The disadvantage of course i that i couldn't reuse any common code between the two parts, and this felt like two completely unrelated puzzles.

After arriving to this eclectic solution, i went browsing on the AoC subreddit to see how other people have solved this, and i found out about a much more clever and elegant approach: scanning each row (or column) and counting how many piles belonging to the loop you go through. If you have passed an odd number of pipes, you're inside the loop, so any tile not belonging to the loop is inside it. It's much simpler, and of course much more tightly connected to part 1. Oh well! 🙃

### Day 11: Cosmic Expansion

Neat puzzle. Part 2 was a nice twist that invalidated my 2D-grid expansion solution for part 1, but it turned out to be be quite simple to implement with similar logic, but acting on the galaxies' positions instead of on the grid itself.

`Iterator::tuple_combinations()` from `itertools` was perfect for easily pairing up all galaxies :)

### Day12: Hot Springs

What a brutal part 2! Hardest puzzle so far.

Part 1 could be solved by brute-forcing all possible spring arrangements. But part 2 increased the input sizes fivefold, which, for an exponential $O(2^n)$ brute-force solution and an $n$ increasing from ~20 (i.e. millions of combinations) to ~100, meant non-termination before my death and the death of all stars in the universe. So i had to think of another approach; i'm not that patient.

Two old tricks ended up doing wonders to cut this existential-crisis-inducing runtime: regular expressions, and caching.

[![]( https://imgs.xkcd.com/comics/perl_problems.png)](https://xkcd.com/1171/)

Regular expressions were used to check whether a row of springs like `?###??..??` can match a given set of damaged spring group numbers like `[3, 2]`. Instead of implementing all the ad-hoc matching logic myself, the `[3, 2]` numbers get converted to the regex `^[.?]*[#?]{3}[.?]+[#?]{2}[.?]*$` that will only match spring rows that could have those groups of damaged springs, like `.###.##..` or `?###??..??`, but not `###..###` or `?###?#..??`. This way, the regex engine takes care of all the complicated matching logic to avoid trying out millions of arrangements that cannot possibly match the group numbers.

Still, even with the regex matching cutting down the recursion branches greatly, the solution was still counting each single possible arrangement individually, which, for a final answer in the order of $10^{15}$, still meant a ridiculous amount of time to run.

The second trick was to cache and reuse results to avoid counting every arrangement individually. For example, for input line `?###???????? 3,2,1` (which is part of the example on the puzzle description), at some point we'll get to compute all arrangements for `.###.##..???`, which are 3. And then we'll get to a similar point but with a different position for the 2nd group: `.###..##.???`. Instead of recalculating all 3 arrangements again, we cache that result the first time and reuse it the second time.

This simple optimization cuts down the time to get to the answer from i don't know how much (i left it running for more than an hour) to half a second. Caches can be hard to get right, but they pay off.

Finally, using Rayon to iterate over all input lines in parallel brought down runtime ~540ms to ~130ms. Not bad at all considering this just mean changing a single `.iter()` call to `.par_iter()`.

I'm incredibly grateful for high-quality crates like `regex` and `rayon` on the Rust ecosystem :)
