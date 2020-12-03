use clap::Clap;
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

#[derive(Copy, Clone, Debug)]
enum Square {
    Empty,
    Tree,
}

struct Topology<'t> {
    squares: &'t Vec<Vec<Square>>,
    height: usize,
    width: usize,
}

struct Slope {
    vertical: usize,
    horizontal: usize,
}

impl<'t> Topology<'t> {
    /// Get the squares on the path of some slope
    fn iter(&'t self, slope: Slope) -> TopologyIterator<'t> {
        TopologyIterator {
            topology: &self,
            slope: slope,
            curr_vertical: 0,
            curr_horizontal: 0,
        }
    }
}

struct TopologyIterator<'t> {
    topology: &'t Topology<'t>,
    slope: Slope,
    curr_vertical: usize,
    curr_horizontal: usize,
}

impl<'t> Iterator for TopologyIterator<'t> {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        if self.curr_vertical >= self.topology.height {
            return None;
        }

        let horizontal = self.curr_horizontal;
        let vertical = self.curr_vertical;

        self.curr_horizontal = self.curr_horizontal + self.slope.horizontal;
        self.curr_vertical = self.curr_vertical + self.slope.vertical;

        Some(self.topology.squares[vertical][horizontal % self.topology.width])
    }
}
