use clap::Clap;
use day03_01::{Slope, Square, Topology};
use std::fs::read_to_string;

#[derive(Clap)]
struct Opts {
    input: String,
}

fn main() -> Result<(), std::io::Error> {
    let opts: Opts = Opts::parse();
    let input = read_to_string(opts.input)?;

    let squares = parse(&input).unwrap();

    let topology = Topology {
        squares: &squares,
        height: squares.len(),
        width: squares.first().unwrap().len(),
    };

    let num_trees = topology
        .iter(Slope {
            horizontal: 3,
            vertical: 1,
        })
        .filter(|square| match square {
            Square::Tree => true,
            Square::Empty => false,
        })
        .count();

    println!("{}", num_trees);

    Ok(())
}

fn parse(input: &str) -> Option<Vec<Vec<Square>>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Some(Square::Empty),
                    '#' => Some(Square::Tree),
                    _ => None,
                })
                .collect()
        })
        .collect()
}
