use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    File::open("hello.txt")?;

    Ok(())
}
