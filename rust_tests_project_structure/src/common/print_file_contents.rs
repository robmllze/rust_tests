use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn print_file_contents(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }
    Ok(())
}
