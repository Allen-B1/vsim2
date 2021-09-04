extern crate vsim2;
use vsim2::core::*;

use crate::ui::*;
use std::sync::{Arc,Weak};
use yew::prelude::*;

pub fn generate<'a>(parties: impl Iterator<Item=(&'a str, u32, u32)> + Clone + 'a) -> Html {
    let total: u32 = parties.clone().map(|(_, size, _)| size).sum();
    html!(
        <div class="parliament" style="--radius:128px">
            {{
                let mut sum: u32 = 0;
                parties.map(|(name, size, clr)|{
                     let html = html!(<div class="slice" title={format!("{}: {}", name, size)} style={format!("--size: {}deg; --off: {}deg; background: {}", (size as f64 / total as f64) * 180.0, (sum as f64 / total as f64) * 180.0, color_to_hex(clr))}></div>);
                    sum += size;
                     html
                }).collect::<Html>()
            }}
            <div class="blocker" style="--size:64px">{total}</div>
        </div>
    )
}