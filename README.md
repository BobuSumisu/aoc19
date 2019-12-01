# Advent of Code 2019

My solutions to the [2019 Advent of Code edition](https://adventofcode.com/) in Rust.

I am a Rust noob so I am probably going to do these challenges using the following process:

1. Write an ugly and inefficient solution that works.
2. Look at other people's solutions to ~~shamelessly copy~~ learn and be inspired by their code.
3. Rewrite my solutions.

To run the code for a solution just jump into a directory and build/run it:

```bash
$ cd day01
$ cargo run --release < input/input.txt
```

The solutions include tests. To run all of them quickly you can do something like:

```bash
$ for f in day??/Cargo.toml; do cargo test --release --manifest-path=$f; done
```

## Inspiration

- [Armin Ronacher](https://github.com/mitsuhiko/aoc19)
- [Andrew Gallant (2018)](https://github.com/BurntSushi/advent-of-code)

## Stuff Learned

### Day 1

[Saturating arithmetic operations](https://doc.rust-lang.org/std/primitive.u64.html#method.saturating_sub) is much
sexier than manually checking for and handling over-/underflow.

Use the [successors](https://doc.rust-lang.org/std/iter/fn.successors.html) function to create an iterator where each successive item
is computed based on the preceding one.

Use [filter_map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map) to, well, filter and map.
Very useful with [ok](https://doc.rust-lang.org/std/result/enum.Result.html#method.ok) which converts a `Result<T, E>` into `Option<T>`
(and thus dicarding the error).
