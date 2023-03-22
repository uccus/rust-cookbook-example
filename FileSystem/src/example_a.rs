use error_chain::error_chain;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};

error_chain!{
    foreign_links{
        IIo(std::io::Error);
    }
}

pub fn test() -> Result<()> {
    let path = "lines.txt";
    let mut output = File::create(path)?;
    write!(output, "you want ❥(^_-)")?;
    let input = File::open(path)?;
    let buffer = BufReader::new(input);
    for line in buffer.lines() {
        println!("{}", line?);
    }
    Ok(())
}