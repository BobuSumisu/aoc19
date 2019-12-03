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

### Day 2

Mainly: don't overengineer! Apart from that I didn't see any major differences (that I liked) in other people's solutions.

A small thing I learned was using `&[T]` over `&Vec<T>` in function parameters.
The only time you want to use `&Vec<T>`
is if you for some reason need to read the vector's capacity.
If you want to change the vector you'd have to use `&mut Vec<T>` anyway (ref. [Steve
Klabnik](https://www.reddit.com/r/rust/comments/8kujd1/newbie_correct_way_to_pass_vector_of_references/)).

### Day 3

The current code is just terrible. It just loops through the of path A and at every points checks it against every
point in path B.

Was thinking about converting the paths to collection of lines and using some line intersection math stuff, but need to
get the time to look into it.

It works for now, but it takes ~12 seconds on my laptop :(.
