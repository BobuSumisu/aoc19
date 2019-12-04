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

- [Andrew "BurntSushi" Gallant (2018)](https://github.com/BurntSushi/advent-of-code)
- [Armin "mitsuhiko" Ronacher](https://github.com/mitsuhiko/aoc19)
- [Marc "noirotm" Noirot](https://github.com/noirotm/advent-of-code-2019)

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

Improved performance quite a bit by using `HashSet` (and the
[intersection](https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.intersection) method) instead of
`Vec`.

Learned to implement the [Hash](https://doc.rust-lang.org/std/hash/trait.Hash.html) and
[PartialEq](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) traits.

Still not sure about `Eq` vs `PartialEq` and `Ord` vs `PartialOrd`. Probably need to see it more in the wild to
understand when and why to use them.

Some other subjects that still are fuzzy: reference vs value, `iter` vs `into_iter`, all the different `ok`, `ok_or`,
`or_else`, `ok_or_else`, `unwrap_or`, `unwrap_or_else`, etc. Probably mixing `Option` and `Result` here.

Lifetimes are still pretty much terra incognita so I basically try avoid them for now. Same for all that trait stuff in
functions (apparently they are called [bounds](https://doc.rust-lang.org/rust-by-example/generics/bounds.html)).

### Day 4

Found a great answer on
[StackOverflow](https://stackoverflow.com/questions/27535289/what-is-the-correct-way-to-return-an-iterator-or-any-other-trait)
on how to return an `Iterator` from a function. To summarize: use [impl
trait](https://github.com/rust-lang/rfcs/blob/master/text/1522-conservative-impl-trait.md) or  `Box` (if the return
type is decided dynamically). Can
also use [newtype](https://doc.rust-lang.org/1.0.0/style/features/types/newtype.html) or [type
aliases](https://doc.rust-lang.org/1.1.0/book/type-aliases.html).
