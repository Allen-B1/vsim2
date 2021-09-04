use crate::core::*;

#[derive(Clone)]
pub struct DMP {
    pub threshold: f32,
}

impl ElectoralMethod for DMP {
    fn district_size(&self) -> u32 {
        2
    }

    fn run(&self, stage: &ElectionStage, r: &ElectionResults, g: &Grouping) -> Result<SeatResult, String> {
        Err("todo".to_owned())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}