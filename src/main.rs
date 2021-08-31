mod core;
mod source;
mod methods;
pub use crate::core::*;

use std::{io, fs};

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open("src-canada.zip")?;
    let (stage, results) = source::canada::from_zip(file)?;

    let out_file = fs::File::create("src-canada-stage.json")?;
    serde_json::to_writer_pretty(out_file, &stage)?;

    Ok(())
}

fn main() {
    println!("Hello, world!");
}
