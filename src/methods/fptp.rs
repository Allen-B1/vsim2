use crate::*;
use std::collections::{HashMap,HashSet};
use std::iter::Iterator;

pub struct FPTP;

impl VotingMethod for FPTP {
    fn district_size(&self) -> u32 {
        1
    }

    fn run(&self, stage: &ElectionStage, r: &ElectionResults, groupings: &Grouping) -> SeatResult {
        let mut successful = HashSet::new();
        for (gid, districts) in groupings.iter() {
            let seats: u8 = districts.iter().map(|&id| stage.districts[&id].seats ).sum();

            let mut items = Vec::new();
            for &district in districts.iter() {
                for (&candidate, &votes) in &r.results[&district].votes {
                    items.push((candidate, votes));
                }
            }

            items.sort_by(|(c, v), (c2, v2)| v2.cmp(v));

            successful.extend(items.iter().map(|(c, _)| *c).take(seats as usize));
        }

        SeatResult{
            seats: successful
        }
    }
}