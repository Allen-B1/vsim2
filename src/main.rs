extern crate vsim2;

use vsim2::*;
use vsim2::core::*;

mod ui;
use std::{io, fs};
use std::collections::HashMap;
extern crate rmp_serde as rmps;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    yew::start_app::<ui::Model>();

    Ok(())
}
