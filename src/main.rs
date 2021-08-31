mod core;
mod source;
mod stats;
mod methods;
pub use crate::core::*;

use std::{io, fs};

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open("src-canada.zip")?;
    let (stage, results) = source::canada::from_zip(file)?;

    let out_file = fs::File::create("src-canada-stage.json")?;
    serde_json::to_writer_pretty(out_file, &stage)?;

    let out_file = fs::File::create("src-canada-results.json")?;
    serde_json::to_writer_pretty(out_file, &results)?;

    Ok(())
}

#[test]
fn stats() -> Result<(), Box<dyn std::error::Error>> {
    let stage: ElectionStage = serde_json::from_reader(fs::File::open("src-canada-stage.json")?)?;
    let results: ElectionResults = serde_json::from_reader(fs::File::open("src-canada-results.json")?)?;

    let method = methods::fptp::FPTP;
    let seats = method.run(&stage, &results);

    let stats = stats::seats_by_party(&stage, &seats);
    for (party, &seats) in stats.iter() {
        if let stats::StatsParty::Party(p) = party {
            print!("{} => {}\n", stage.parties[*p].name, seats);
        } else {
            print!("IND => {}\n", seats);
        }
    }

    Ok(())
}

fn main() {
    println!("Hello, world!");
}
