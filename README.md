# Advent of Code 2022

Using [Advent of Code 2022](https://adventofcode.com/2022/about) as a chance to learn Rust.

## To not forget

* Execute project as `cargo run -p day01 -- day01/data/input.txt`
* Run Clippy to perform official linting (like - `cargo clippy -p day03`)

## Copilot prompt

```text
Act as a Senior Rust Engineer performing a thorough code review. Analyze this file for:

1. Idiomatic Rust: Suggest where I can use more 'Rusty' patterns (e.g., functional iterators over loops, match over if/else).
2. Performance: Identify unnecessary .clone(), heap allocations, or areas where Borrow could replace owned types.
3. Error Handling: Point out where unwrap() or expect() should be replaced with proper Result propagation.
4. Safety: Highlight any unsafe blocks that lack sufficient justification or potential logic bugs that could cause panics.
5. Review trait bounds for flexibility and check if 'Static Dispatch' is preferred over 'Dynamic Dispatch' here.

Please be concise and provide code snippets for your suggestions.
```
