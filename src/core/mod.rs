use std::any::Any;
use std::collections::{HashMap,HashSet};
use serde::{Serialize,Deserialize,Serializer,Deserializer};
use serde::ser::{SerializeTuple};
use serde::de::{self, Visitor};
use std::fmt;

extern crate rmp_serde as rmps;

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
/// If the country does not have electorally-relevant provinces or similar,
/// use one `Area` for the entire country.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Area {
    pub name: String,

    /// `District`s inside of this `Area`
    pub districts: HashSet<DistrictID>,

    /// Candidates associated with the given area.
    /// See `Candidate` for more information on whether to use `Area::candidates` 
    /// or `District::candidates`.
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    #[serde(default)]
    pub candidates: HashSet<CandidateID>,
}

/// Represents an electoral district.
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct District {
    pub name: String,
    pub seats: u8,
    pub area: AreaID,

    /// Candidates associated with the given district.
    /// See `Candidate` for more information on whether to use `Area::candidates` 
    /// or `District::candidates`.
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    #[serde(default)]
    pub candidates: HashSet<CandidateID>,
}

/// Represents a candidate.
///
/// Candidates running in district-wide elections (e.g. on a local party list,
/// in local FPTP elections) should be put in `District::candidate`.
/// Candidates running in area-wide elections (e.g. on a regional party list)
/// should be put in `Area::district`. Candidates that run in both kinds of
/// elections (e.g. for MMP - regional party list + local election)
/// should be put in both `District::candidates` and `Area::candidates`.
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Candidate {
    /// Name of the candidate.
    /// `None` if the name data isn't available.
    pub name: Option<String>,

    /// Party of the candidate.
    /// `None` if the candidate is an independent.
    pub party: Option<PartyID>,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Party {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: PartyType,
    pub color: u32,
}

/// Represents the category of a party.
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

pub type Groupings = HashMap<u32, Grouping>;

//= Data after the election =//

/// Represents the election results.
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct ElectionResults {
    pub districts: HashMap<DistrictID, DistrictResults>,
    pub date: Date,
}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq, Eq, Hash, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum PartyListSource { Area, District }

/// Represents results in one electoral district.
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct DistrictResults {
    /// Votes for a party. This should
    /// only be used in closed party-list PR
    /// or something similar, where the voter
    /// votes only for a party. In open-list
    /// elections, `candidate_votes` should be
    /// used instead.
    pub party_votes: HashMap<PartyID, u32>,
    pub party_list_source: PartyListSource,

    pub candidate_votes: HashMap<CandidateID, u32>,
}

//= Data after voting method =//
#[derive(Debug,Clone)]
pub struct SeatResult {
    pub seats: HashSet<CandidateID>,
}

pub trait ElectoralMethod: dyn_clone::DynClone + std::any::Any {
    fn district_size(&self) -> u32;
    fn run(&self, stage: &ElectionStage, r: &ElectionResults, g: &Grouping) -> Result<SeatResult, String>;

    fn as_any(&self) -> &dyn Any;
}

dyn_clone::clone_trait_object!(ElectoralMethod);

pub fn encode(w: &mut (impl std::io::Write + ?Sized), data: (&ElectionStage, &ElectionResults, &HashMap<u32, Grouping>)) -> Result<(), rmps::encode::Error> {
    data.serialize(&mut rmps::Serializer::new(w))
}

pub fn decode(r: &mut (impl std::io::Read + ?Sized)) -> Result<(ElectionStage, ElectionResults, HashMap<u32, Grouping>), rmps::decode::Error> {
    Deserialize::deserialize(&mut rmps::Deserializer::new(r))
}

pub mod utils {
    use crate::core::*;
    pub fn party_candidates<'a>(stage: &'a ElectionStage, party: PartyID, candidates: impl Iterator<Item=CandidateID>  + 'a) -> impl Iterator<Item=CandidateID> + 'a {
        candidates.filter(move |&p| stage.candidates[&p].party == Some(party))
    }

    pub fn party_list<'a>(stage: &'a ElectionStage, party: PartyID, source: PartyListSource, district: DistrictID) -> impl Iterator<Item=CandidateID> + 'a {
        utils::party_candidates(stage, party, match source {
            PartyListSource::Area => stage.areas[&stage.districts[&district].area].candidates.iter(),
            PartyListSource::District => stage.districts[&district].candidates.iter()
        }.map(|&x| x))
    }

    pub fn seats_by_party(stage: &ElectionStage, seats: &SeatResult) -> HashMap<Option<PartyID>, usize> {
        let mut parties = HashMap::new();
        for seat_idx in &seats.seats {
            let item = match stage.candidates[seat_idx].party {
                Some(party) => Some(party),
                None => None
            };

            let new_count = parties.get(&item).unwrap_or(&0) + 1;
            parties.insert(item, new_count);
        }

        parties
    }
}
