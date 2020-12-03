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

    let slopes = vec![
        Slope {
            horizontal: 1,
            vertical: 1,
        },
        Slope {
            horizontal: 3,
            vertical: 1,
        },
        Slope {
            horizontal: 5,
            vertical: 1,
        },
        Slope {
            horizontal: 7,
            vertical: 1,
        },
        Slope {
            horizontal: 1,
            vertical: 2,
        },
    ];

    let squares = parse(&input).unwrap();

    let topology = Topology {
        squares: &squares,
        height: squares.len(),
        width: squares.first().unwrap().len(),
    };

    let result = slopes
        .iter()
        .map(|slope| {
            topology
                .iter(slope)
                .filter(|square| match square {
                    Square::Tree => true,
                    Square::Empty => false,
                })
                .count()
        })
        .product::<usize>();

    println!("{}", result);

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
