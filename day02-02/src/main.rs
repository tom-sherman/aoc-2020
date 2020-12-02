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
    pos_1: i32,
    pos_2: i32,
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
            Regex::new(r"(?P<pos_1>\d+)-(?P<pos_2>\d+) (?P<letter>[a-z]): (?P<password>[a-z]+)")
                .unwrap();
    }

    let captures = RE.captures(string);

    Row {
        pos_1: unwrap_match(&captures, "pos_1")
            .unwrap()
            .parse::<i32>()
            .unwrap(),
        pos_2: unwrap_match(&captures, "pos_2")
            .unwrap()
            .parse::<i32>()
            .unwrap(),
        letter: unwrap_match(&captures, "letter").unwrap(),
        password: unwrap_match(&captures, "password").unwrap(),
    }
}

fn test_row(row: &Row) -> bool {
    let chars: Vec<char> = row.password.chars().collect();
    let pos_1 = chars.get(row.pos_1 as usize - 1).unwrap().to_string();
    let pos_2 = chars.get(row.pos_2 as usize - 1).unwrap().to_string();

    (pos_1 == row.letter) ^ (pos_2 == row.letter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_row() {
        assert_eq!(
            test_row(&Row {
                pos_1: 1,
                pos_2: 3,
                letter: "a",
                password: "abcde"
            }),
            true
        );

        assert_eq!(
            test_row(&Row {
                pos_1: 1,
                pos_2: 3,
                letter: "b",
                password: "cdefg"
            }),
            false
        );

        assert_eq!(
            test_row(&Row {
                pos_1: 2,
                pos_2: 9,
                letter: "c",
                password: "ccccccccc"
            }),
            false
        );
    }
}
