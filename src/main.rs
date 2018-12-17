extern crate cel_rs;

use std::io;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn main() -> Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    let parsed = cel_rs::parsers::parse(&buf)?;
    println!("{:?}", parsed);
    Ok(())
}
