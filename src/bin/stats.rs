extern crate vsim2;
use vsim2::core::*;
use vsim2::*;

use std::{env,fs};

fn terminal_color(clr: u32, fg: bool) -> String {
    format!("\x1b[{}8;2;{};{};{}m", if fg { 3 } else { 4 }, (clr & 0xff0000) >> 16, (clr & 0x00ff00) >> 8, clr & 0x0000ff)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args();
    let path = args.skip(1).next().ok_or("input file not specified")?;

    let mut file = fs::File::open(path)?;
    let (stage, results, groupings) = core::decode(&mut file)?;

    let mut sizes: Vec<_> = groupings.keys().collect();
    sizes.sort();

    let method = methods::fptp::FPTP;
    let seats = method.run(&stage, &results, &groupings[&sizes[0]])?;

    let stats = utils::seats_by_party(&stage, &seats);
    let mut stats_vec: Vec<_> = stats.iter().collect();
    stats_vec.sort_by(|a, b| b.1.cmp(a.1));

//    print!("{}\n", terminal_color(0xffffff, false));
    for (party, &seats) in stats_vec.iter() {
        if let Some(p) = party {
            print!("{}{}{} => {}\n",
                terminal_color(stage.parties[&p].color, true),
                stage.parties[&p].name,
                terminal_color(0x000000, true),
                seats);
        } else {
            print!("{}<ind>{} => {}\n", terminal_color(0xaaaaaa, true), seats,                 terminal_color(0x000000, true));
        }
    }

    print!("\x1b[0m\n\n");

    Ok(())
}