use crate::core::*;
use std::collections::{HashMap,HashSet};
use std::iter::Iterator;

#[derive(Clone)]
pub struct FPTP;

impl ElectoralMethod for FPTP {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn district_size(&self) -> u32 {
        1
    }

    fn run(&self, stage: &ElectionStage, r: &ElectionResults, groupings: &Grouping) -> Result<SeatResult, String> {
        let mut successful = HashSet::new();
        for (gid, districts) in groupings.iter() {
            let seats: u8 = districts.iter().map(|&id| stage.districts[&id].seats).sum();

            let mut total_candidates_votes = Vec::new();
            for &district in districts.iter() {
                let dres = &r.districts[&district];

                if !dres.candidate_votes.is_empty() { // District uses candidate votes
                    for (&candidate, &votes) in dres.candidate_votes.iter() {
                        total_candidates_votes.push((candidate, votes));
                    }    
                } else { // District only has party votes. In this case, we use the first candidate in the party list.
                    let source = dres.party_list_source;
                    for (&party, &votes) in dres.party_votes.iter() {
                        let mut candidates: Vec<CandidateID> = utils::party_list(stage, party, source, district).collect();
                        candidates.sort();
                        total_candidates_votes.push((*candidates.get(0).ok_or(format!("party {} does not have any candidates on list", party))?, votes));
                    }
                }
            }

            total_candidates_votes.sort_by(|(c, v), (c2, v2)| v2.cmp(v));

            successful.extend(total_candidates_votes.iter().map(|(c, _)| *c).take(seats as usize));
        }

        Ok(SeatResult{
            seats: successful
        })
    }
}