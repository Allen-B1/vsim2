pub mod stats;

use std::collections::{HashMap,HashSet};
use serde::{Serialize,Deserialize,Serializer,Deserializer};
use serde::ser::{SerializeTuple};
use serde::de::{self, Visitor};
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Date {
    pub year: u32,
    pub month: u8,
    pub day: u8
}

impl Date {
    pub fn new(year: u32, month: u8, day: u8) -> Date {
        Date { year, month, day }
    }
}

impl Serialize for Date {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut tup = serializer.serialize_tuple(3)?;
        tup.serialize_element(&self.year)?;
        tup.serialize_element(&self.month)?;
        tup.serialize_element(&self.day)?;
        tup.end()
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct DateVisitor;

        impl<'de> Visitor<'de> for DateVisitor {
            type Value = Date;

            
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a year, month, and day value")
            }

            fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Date, A::Error> {
                let year = seq.next_element::<u32>()?.ok_or(<A::Error as de::Error>::custom("year value not present"))?;
                let month = seq.next_element::<u8>()?.ok_or(<A::Error as de::Error>::custom("month value not present"))?;
                let day = seq.next_element::<u8>()?.ok_or(<A::Error as de::Error>::custom("day value not present"))?;
                Ok(Date{year, month, day})
            }
        }

        deserializer.deserialize_tuple(3, DateVisitor)
    }
}

pub type DistrictID = u32;
pub type AreaID = u16;
pub type CandidateID = u32;
pub type PartyID = u8;

/// Represents data before the election.
#[derive(Debug,Clone, Serialize,Deserialize)]
pub struct ElectionStage {
    pub districts: HashMap<DistrictID, District>,
    pub candidates: HashMap<CandidateID, Candidate>,
    pub parties: HashMap<PartyID, Party>,
    pub areas: HashMap<AreaID, Area>,
}

/// Represents a set of districts, like a province or state.
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Area {
    pub name: String,
    pub districts: HashSet<DistrictID>,

    /// Party-list candidates. Use an empty `HashMap` is the given country does not have party lists.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub candidates: HashMap<PartyID, HashSet<CandidateID>>
}

/// Represents an electoral district.
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct District {
    pub name: String,
    pub candidates: HashSet<CandidateID>,
    pub seats: u8,
}

/// Represents a candidate.
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Candidate {
    pub name: Option<String>,
    pub party: Option<PartyID>,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub  struct Party {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: PartyType,
    pub color: u32,
}

#[derive(Debug,Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum PartyType {
    Left = 0,
    SocialDemocratic = 1,
    Green = 2,
    Liberal = 3,
    Other = 4,
    Conservative = 5,
    Fascist = 6
}

impl Default for PartyType {
    fn default() -> Self {
        PartyType::Other
    }
}

/// Format: `[set of districts]`
#[repr(transparent)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Grouping (pub Vec<HashSet<DistrictID>>);

impl Grouping {
    pub fn candidates(&self, grouping: usize, stage: &ElectionStage) -> HashSet<CandidateID> {
        let districts = &self.0[grouping];
        districts.iter().map(|&district| stage.districts[&district].candidates.iter()).flatten().map(|&x| x).collect()
    }

    pub fn keys(&self) -> impl Iterator<Item=usize> {
        0..self.0.len()
    }

    pub fn values(&self) -> impl Iterator<Item=&HashSet<DistrictID>> {
        self.0.iter()
    }

    pub fn iter(&self) -> impl Iterator<Item=(usize, &HashSet<DistrictID>)> {
        self.0.iter().enumerate()
    }
}

//= Data after the election =//
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct ElectionResults {
    pub results: HashMap<DistrictID, DistrictResult>,
    pub date: Date,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct DistrictResult {
    pub votes: HashMap<CandidateID, u32>,
    pub list_votes: HashMap<PartyID, u32>,
}

//= Data after voting method =//
#[derive(Debug,Clone)]
pub struct SeatResult {
    pub seats: HashSet<CandidateID>,
}

pub trait VotingMethod {
    fn district_size(&self) -> u32;
    fn run(&self, stage: &ElectionStage, r: &ElectionResults, g: &Grouping) -> SeatResult;
}