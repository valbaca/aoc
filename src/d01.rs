//! Day 01
//! Coming back to Rust quite "cold" and mostly fumbling my way through.
//! But also got to a working answer quite quick.
//!
//! TIL: for iter to Vec, just specify the type and use .collect()
//!
use std::{fs, io};

pub fn part1() -> Result<(), io::Error>{
    let file = fs::read_to_string("in/d01")?;
    let mut lines = file.lines();
    let line = lines.next().unwrap();
    let mut sum = 0;
    let mut char_vec : Vec<char> = line.chars().collect();
    char_vec.push(*char_vec.first().unwrap()); // put first at end
    for pair in char_vec.windows(2) {
        if pair[0] == pair[1] {
            sum += char::to_digit(pair[0], 10).unwrap();
        }
    }
    println!("{}", sum);
    Ok(())
}

pub fn part2() -> Result<(), io::Error>{
    let file = fs::read_to_string("in/d01")?;
    let mut lines = file.lines();
    let line = lines.next().unwrap();
    let mut sum = 0;
    let char_vec: Vec<char> = line.chars().collect();
    let half = char_vec.len() / 2;
    for i in 0..char_vec.len() {
        if char_vec[i] == char_vec[(i + half) % char_vec.len()] {
            sum += char::to_digit(char_vec[i], 10).unwrap();
        }
    }
    println!("{}", sum);
    Ok(())
}

