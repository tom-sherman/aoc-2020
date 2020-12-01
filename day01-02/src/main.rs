use clap::Clap;
use std::fs::read_to_string;

#[derive(Clap)]
struct Opts {
    input: String,
}

fn main() -> Result<(), std::io::Error> {
    let opts: Opts = Opts::parse();
    let input = read_to_string(opts.input)?;
    let entries: Vec<(usize, i32)> = input
        .lines()
        .map(|line| line.parse::<i32>().expect("Expected an int on each line."))
        .enumerate()
        .collect();

    for (a_index, a_entry) in entries.iter() {
        for (b_index, b_entry) in entries.iter() {
            if b_index == a_index || a_entry + b_entry > 2020 {
                continue;
            }

            for (c_index, c_entry) in entries.iter() {
                if c_index == b_index || c_index == a_index {
                    continue;
                }

                if a_entry + b_entry + c_entry == 2020 {
                    println!("{}", a_entry * b_entry * c_entry);
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}
