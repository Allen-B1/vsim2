use crate::core::*;
use std::collections::HashMap;

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub enum StatsParty {
    Party(PartyID),
    Ind(CandidateID)
}

pub fn seats_by_party(stage: &ElectionStage, seats: &SeatResult) -> HashMap<StatsParty, usize> {
    let mut parties: HashMap<StatsParty, usize> = HashMap::new();
    for seat_idx in &seats.seats {
        let item = match stage.candidates[seat_idx].party {
            Some(party) => StatsParty::Party(party),
            None => StatsParty::Ind(*seat_idx)
        };

        let new_count = parties.get(&item).unwrap_or(&0) + 1;
        parties.insert(item, new_count);
    }

    return parties;
}