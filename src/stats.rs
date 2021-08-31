use crate::*;
use std::collections::HashMap;

#[derive(Clone,PartialEq,Eq)]
pub enum StatsParty {
    Party(usize),
    Ind(usize)
}

fn seats_by_party(stage: &ElectionStage, seats: &SeatResult) -> HashMap<StatsParty, usize> {
    let parties: HashMap<StatsParty, usize> = HashMap::new();
    for seat_idx in &seats.seats {
        let item = match stage.candidates[seat_idx].party {
            Some(party) => StatsParty::Party(party),
            None => StatsParty::Ind(seat_idx)
        };

        let newCount = parties.get(item).unwrap_or(0) + 1;
        parties.insert(item, newCount);
    }

    return parties;
}