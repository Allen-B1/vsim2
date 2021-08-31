use std::collections::{HashMap,HashSet};

pub struct Date {
    year: u32,
    month: u8,
    day: u8
}

//= Data before the election =//
pub  struct ElectionStage {
    districts: Vec<District>,
    areas: Vec<Area>,
    candidates: Vec<Candidate>,
    parties: Vec<Party>,
}

pub  struct Area {
    name: String,
    districts: Vec<usize>,
}

pub  struct District {
    name: String,
    candidates: Vec<usize>,
    size: u8,
}

pub  struct Candidate {
    name: String,
    party: usize,
}

pub  struct Party {
    name: String,
    type_: PartyType,
    color: u32,
}

pub enum PartyType {
    Left,
    SocialDemocratic,
    Green,
    Liberal,
    Conservative,
    Fascist,
    Other
}

impl Default for PartyType {
    fn default() -> Self {
        PartyType::Other
    }
}

//= Data after the election =//
pub struct ElectionResults {
    time: String,
    results: Vec<DistrictResult>,
    date: Date,
}
pub struct DistrictResult {
    votes: HashMap<usize, u32>,
}

//= Data after voting method =//
pub struct SeatResult {
    seats: HashSet<usize>,
}

pub trait VotingMethod {
    fn run(&self, stage: &ElectionStage, r: &ElectionResults) -> SeatResult;
}