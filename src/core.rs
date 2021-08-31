use std::collections::{HashMap,HashSet};
use serde::{Serialize,Deserialize,Serializer,Deserializer};
use serde::ser::{SerializeTuple};
use serde::de::{self, Visitor};
use std::fmt;

#[derive(Copy, Clone)]
pub struct Date {
    pub year: u32,
    pub month: u8,
    pub day: u8
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

//= Data before the election =//
#[derive(Serialize,Deserialize)]
pub  struct ElectionStage {
    pub districts: Vec<District>,
    pub areas: Vec<Area>,
    pub candidates: Vec<Candidate>,
    pub parties: Vec<Party>,
}

#[derive(Clone,Serialize,Deserialize)]
pub  struct Area {
    pub name: String,
    pub districts: HashSet<usize>,
}

#[derive(Clone,Serialize,Deserialize)]
pub  struct District {
    pub name: String,
    pub candidates: HashSet<usize>,
    pub size: u8,
}

#[derive(Clone,Serialize,Deserialize)]
pub  struct Candidate {
    pub name: String,
    pub party: Option<usize>,
}

#[derive(Clone,Serialize,Deserialize)]
pub  struct Party {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: PartyType,
    pub color: u32,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
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
#[derive(Clone,Serialize,Deserialize)]
pub struct ElectionResults {
    pub results: Vec<DistrictResult>,
    pub date: Date,
}
#[derive(Clone,Serialize,Deserialize)]
pub struct DistrictResult {
    pub votes: HashMap<usize, u32>,
}

//= Data after voting method =//
#[derive(Clone)]
pub struct SeatResult {
    pub seats: HashSet<usize>,
}

pub trait VotingMethod {
    fn run(&self, stage: &ElectionStage, r: &ElectionResults) -> SeatResult;
}