# Libaocparser_rs
An AOC input parser for Rust

## Install
Add this line to Cargo.toml under dependencies.
```
libaocparser_rs = { git = "https://github.com/Cryxtalix/libaocparser_rs" }
```

## Usage
Use `AocParser::new` to add input file.

`AocParser::slice_as_type` to convert specific lines into a particular type.
Type must implement `std::str::FromStr`.

Example

```
use libaocparser_rs::AocParser;
use std::str::FromStr;

#[derive(Debug)]
struct TestType {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct TestErr;

impl FromStr for TestType {
    type Err = TestErr;

    fn from_str(s: &str) -> Result<Self, TestErr> {
        let mut parts = s.split_whitespace();

        let x = parts.next().ok_or(TestErr)?.parse::<u32>().map_err(|_| TestErr)?;
        let y = parts.next().ok_or(TestErr)?.parse::<u32>().map_err(|_| TestErr)?;
        let z = parts.next().ok_or(TestErr)?.parse::<u32>().map_err(|_| TestErr)?;
        //                     ^ next part is None
        //                                                     ^ parsing failed

        if parts.next().is_some() {
            // More parts than expected!
            return Err(TestErr)
        }

        Ok(TestType { x, y, z })
    }
}

fn main() {
    let aoc = AocParser::new("inputs/day1/input.txt").unwrap();
    let test: Vec<TestType> = aoc.slice_as_type(Some(3), Some(6)).unwrap();

    assert_eq!(aoc.size, 250);
    assert_eq!(test.len(), 3);
}
```
