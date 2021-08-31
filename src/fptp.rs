use crate::*;
use std::collections::{HashMap,HashSet};
use std::iter::Iterator;

struct FPTP;

impl VotingMethod for FPTP {
    fn run(&self, stage: &ElectionStage, r: &ElectionResults) -> SeatResult {
        let successful = HashSet::new();
        for (i, district) in stage.districts.iter().enumerate() {
            let (candidate, votes) = r.results[i].votes.iter().reduce(|(candidate1, votes1), (candidate2, votes2)| if votes1 > votes2 { (candidate1, votes1) } else { (candidate2, votes2) });
            successful.insert(candidate);
        }
        SeatResult{
            seats: successful,
        }
    }
}