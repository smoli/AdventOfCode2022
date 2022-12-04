[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[Advent of Code 2022](https://adventofcode.com/2022/)
=====================================================
I use this to [learn](https://doc.rust-lang.org/book/title-page.html) [Rust](https://www.rust-lang.org/).

I'll see how far I get in terms of the puzzles themselves. I'll try to write some key learnings/struggles 
for each day hoping that once I progressed a bit I'll look at the first days and think: "Well, of course! Noob!"

Since I use this as a learning experience, solutions might be over-engineered 😁

# Day 1

Getting used to the basic syntax and the basics of tooling

# Day 2

Struggling with memory management.

```rust
fn read_input(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path)
        .expect("File not found");

    let lines = contents.split("\r\n");

    let mut r: Vec<String> = vec![];
    for line in lines {
        r.push(String::from(line));
    }

    return r;
}
```

The only way I found to abstract away the basic reading of the input files is to copy the lines into a new vector
creating new `String`s from the `Slice`s the `split` method creates since the slices reference data in the local variable
`contents` that will be out of scope by the end of the function.

# Day 3

Made a local crate. Works great, but no code completion in the IDE. 

# Day 4

I have the feeling I still do too much work for extracting data from strings :-(

I use this to split a string like `12-24,54-67` into its four parts. Will look at regular expression in Rust.

```rust

fn get_ranges(inp: &String) -> Vec<Range> {
    let ranges = inp.split(",").collect::<Vec<&str>>();

    let mut result: Vec<Range> = vec![];

    if ranges.len() == 2 {
        let mut parts = ranges[0].split("-").collect::<Vec<&str>>();

        if parts.len() == 2 {
            result.push(Range { a: parts[0].parse().unwrap(), b: parts[1].parse().unwrap()})
        }

        parts = ranges[1].split("-").collect::<Vec<&str>>();

        if parts.len() == 2 {
            result.push(Range { a: parts[0].parse().unwrap(), b: parts[1].parse().unwrap()})
        }
    }

    result
}

```
)

# License

See [here](LICENSE)
