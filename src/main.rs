use std::{io};

mod read_lines;
mod d01;

fn main() -> Result<(), io::Error> {
    d01::part1()?;
    d01::part2()?;
    Ok(())
}


