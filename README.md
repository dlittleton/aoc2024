# aoc2024

Advent of Code 2024

https://adventofcode.com/2024

## Template

```
use aoc2024::sample;

fn main() {
    aoc2024::run(part1, None);
}

fn part1(input: &str) -> String {
    input.to_string()
}

sample! {
    r"",
    part1 = ""
}
```

## Logging

To enable logging while running, pass the `-v` flag. Multiple occurences will
increase log level.

To enable logging during tests, set the RUST_LOG environment variable.
