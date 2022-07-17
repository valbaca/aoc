use std::{fs, io};

// Example of reading in file
#[allow(dead_code)]
pub fn read_lines() -> Result<(), io::Error> {
    let lines = fs::read_to_string("in/howdy.txt")?;
    for line in lines.lines() {
        println!("{}", line);
    }
    Ok(())
}
