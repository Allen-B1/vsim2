use crate::*;
pub struct DMP {
    threshold: f32,
}

impl VotingMethod for DMP {
    fn district_size(&self) -> u32 {
        2
    }

    fn run(&self, stage: &ElectionStage, r: &ElectionResults, g: &Grouping) -> SeatResult {
        todo!()
    }
}