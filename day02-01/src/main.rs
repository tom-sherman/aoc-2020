#[macro_use]
extern crate lazy_static;
use clap::Clap;
use regex::{Captures, Regex};
use std::fs::read_to_string;

#[derive(Clap)]
struct Opts {
    input: String,
}

fn main() -> Result<(), std::io::Error> {
    let opts: Opts = Opts::parse();
    let input = read_to_string(opts.input)?;

    let count_valid = input
        .lines()
        .map(|l| parse(l))
        .filter(|r| test_row(r))
        .count();

    println!("{}", count_valid);
    Ok(())
}

#[derive(Debug)]
struct Row<'t> {
    letter: &'t str,
    min_occurances: i32,
    max_occurances: i32,
    password: &'t str,
}

fn unwrap_match<'t>(captures: &Option<Captures<'t>>, name: &str) -> Option<&'t str> {
    match captures {
        Some(cap) => cap.name(name).map(|m| m.as_str()),
        None => None,
    }
}

fn parse<'t>(string: &'t str) -> Row {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<letter>[a-z]): (?P<password>[a-z]+)")
                .unwrap();
    }

    let captures = RE.captures(string);

    Row {
        min_occurances: unwrap_match(&captures, "min")
            .unwrap()
            .parse::<i32>()
            .unwrap(),
        max_occurances: unwrap_match(&captures, "max")
            .unwrap()
            .parse::<i32>()
            .unwrap(),
        letter: unwrap_match(&captures, "letter").unwrap(),
        password: unwrap_match(&captures, "password").unwrap(),
    }
}

fn test_row(row: &Row) -> bool {
    let mut occurances = 0;

    for c in row.password.chars().map(|c| c.to_string()) {
        if &c == row.letter {
            occurances += 1;
        }

        if occurances > row.max_occurances {
            return false;
        }
    }

    occurances >= row.min_occurances
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_row() {
        assert_eq!(
            test_row(&Row {
                min_occurances: 1,
                max_occurances: 3,
                letter: "a",
                password: "abcde"
            }),
            true
        );

        assert_eq!(
            test_row(&Row {
                min_occurances: 1,
                max_occurances: 3,
                letter: "b",
                password: "cdefg"
            }),
            false
        );

        assert_eq!(
            test_row(&Row {
                min_occurances: 2,
                max_occurances: 9,
                letter: "c",
                password: "ccccccccc"
            }),
            true
        );
    }
}
