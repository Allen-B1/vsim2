extern crate vsim2;
use vsim2::core::*;
use vsim2::*;

use std::{io, fs, env};
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next();
    let country = args.next().ok_or("missing country")?;

    let (dir, (stage, results, groupings)) = 
    match country.as_str() {
        "canada" => {
            let file = fs::File::open("dataset/canada-2019/src.zip")?;
            ("canada-2019", source::canada::from_zip(file, Date::new(2019, 10, 21))?)
        },
        "germany" => {
            let file = fs::File::open("dataset/germany-2017/src.csv")?;
            ("germany-2017", source::germany::from_csv(file)?)
        }
        _  => {
            return Err("unknown country".into());
        }
    };

    let out_file = fs::File::create(format!("dataset/{}/stage.json", dir))?;
    serde_json::to_writer_pretty(out_file, &stage)?;

    let out_file = fs::File::create(format!("dataset/{}/results.json", dir))?;
    serde_json::to_writer_pretty(out_file, &results)?;

    let out_file = fs::File::create(format!("dataset/{}/groupings.json", dir))?;
    serde_json::to_writer_pretty(out_file, &groupings)?;

    let mut out_file = fs::File::create(format!("dataset/{}/data.elc", dir))?;
    core::encode(&mut out_file, (&stage, &results, &groupings))?;

    Ok(())
}