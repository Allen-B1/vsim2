pub mod core;
pub mod source;
pub mod methods;

mod ui;
use std::{io, fs};
use crate::core::*;
use std::collections::HashMap;

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open("dataset/canada-2019/src.zip")?;
    let (stage, results, groupings) = source::canada::from_zip(file, Date::new(2019, 10, 21))?;

    let out_file = fs::File::create("dataset/canada-2019/stage.json")?;
    serde_json::to_writer_pretty(out_file, &stage)?;

    let out_file = fs::File::create("dataset/canada-2019/results.json")?;
    serde_json::to_writer_pretty(out_file, &results)?;

    let out_file = fs::File::create("dataset/canada-2019/groupings.json")?;
    serde_json::to_writer_pretty(out_file, &groupings)?;


    Ok(())
}

#[test]
fn stats() -> Result<(), Box<dyn std::error::Error>> {
    let stage: ElectionStage = serde_json::from_reader(fs::File::open("dataset/canada-2019/stage.json")?)?;
    let results: ElectionResults = serde_json::from_reader(fs::File::open("dataset/canada-2019/results.json")?)?;
    let groupings: HashMap<u32, Grouping> = serde_json::from_reader(fs::File::open("dataset/canada-2019/groupings.json")?)?;

    let method = methods::fptp::FPTP;
    let seats = method.run(&stage, &results, &groupings[&1]);

    let stats = stats::seats_by_party(&stage, &seats);
    for (party, &seats) in stats.iter() {
        if let stats::StatsParty::Party(p) = party {
            print!("{} => {}\n", stage.parties[p].name, seats);
        } else {
            print!("IND => {}\n", seats);
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    yew::start_app::<ui::Model>();

    Ok(())
}
