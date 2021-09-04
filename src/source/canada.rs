use std::io;
use serde::{Deserialize};
use std::collections::{HashMap,HashSet};
use crate::core::*;

#[derive(Deserialize)]
struct PollRecord {
    #[serde(rename = "Electoral District Number/Numéro de circonscription")]
    district_id: DistrictID,

    #[serde(rename = "Electoral District Name_English/Nom de circonscription_Anglais")]
    district_name: String,

    #[serde(rename = "Candidate’s First Name/Prénom du candidat")]
    candidate_name_first: String,

    #[serde(rename = "Candidate’s Middle Name/Second prénom du candidat")]
    candidate_name_middle: String,

    #[serde(rename = "Candidate’s Family Name/Nom de famille du candidat")]
    candidate_name_last: String,

    #[serde(rename = "Political Affiliation Name_English/Appartenance politique_Anglais")]
    party: String,

    #[serde(rename = "Candidate Poll Votes Count/Votes du candidat pour le bureau")]
    votes: u32
}

pub fn from_zip(reader: impl io::Read + io::Seek, date: Date) -> Result<(ElectionStage, ElectionResults, HashMap<u32, Grouping>), Box<dyn std::error::Error>> {
    let mut archive = zip::ZipArchive::new(reader)?; 
    let mut records: Vec<PollRecord> = Vec::new();
    for i in 0..archive.len() {
        let csv_file = archive.by_index(i)?;

        let mut rdr = csv::Reader::from_reader(csv_file);
        for result in rdr.deserialize::<PollRecord>() {
            records.push(result?);
        }
    }

    let mut parties: HashMap<String, Party> = HashMap::new();
    for record in records.iter() {
        if let None = parties.get(&record.party) {
            let data = match record.party.as_str() {
                "Conservative" => Party {
                    name: "Conservative".to_string(),
                    color: 0x14294d,
                    type_: PartyType::Conservative,
                },
                "Liberal" => Party {
                    name: "Liberal".to_string(),
                    color: 0xda121a,
                    type_: PartyType::Liberal,
                },
                "NDP-New Democratic Party" => Party {
                    name: "New Democrats".to_string(),
                    color: 0xef7c00,
                    type_: PartyType::SocialDemocratic,
                },
                "Bloc Québécois" => Party {
                    name: "Bloc Québécois".to_string(),
                    color: 0x42b7bf,
                    type_: PartyType::Other,
                },
                "Green Party" => Party {
                    name: "Green".to_string(),
                    color: 0x3d9b35,
                    type_: PartyType::Green,
                },
                "Independent" => continue,
                _ => Party {
                    name: record.party.to_string(),
                    color: 0xaaaaaa,
                    type_: PartyType::Other,
                }
            };
            parties.insert(record.party.to_string(), data);
        }
    }

    let parties_ids: HashMap<String, PartyID> = parties.keys().map(Clone::clone).enumerate().map(|(a, b)| (b, a as PartyID)).collect();
    let parties: HashMap<PartyID, Party> = parties.into_iter().map(|(s, party)| (parties_ids[&s], party)).collect();

    let mut candidates: HashMap<String, Candidate> = HashMap::new();
    let mut candidates_votes: HashMap<String, u32> = HashMap::new();
    for record in records.iter() {
        let name = format!("{}{}{} {}", &record.candidate_name_first, if record.candidate_name_middle.is_empty() { "" } else { " " }, &record.candidate_name_middle, &record.candidate_name_last);

        if let None = candidates.get(&name) {
            candidates.insert(name.clone(), Candidate {
                name: Some(name.clone()),
                party: parties_ids.get(&record.party).map(|r| *r),
            });
        }

        *candidates_votes.entry(name).or_insert(0) += record.votes;
    }
    let candidates_ids: HashMap<String, CandidateID> = candidates.keys().map(Clone::clone).enumerate().map(|(a, b)| (b, a as CandidateID)).collect();
    let candidates: HashMap<CandidateID, Candidate> = candidates.into_iter().map(|(name, candidate)| (candidates_ids[&name], candidate)).collect();

    let mut districts: HashMap<DistrictID, District> = HashMap::new();
    for record in records.iter() {
        if let None = districts.get(&record.district_id) {
            districts.insert(record.district_id, District {
                area: (record.district_id / 1000) as AreaID,
                name: record.district_name.clone(),
                candidates: HashSet::new(),
                seats: 1,                
            });
        }

        let name = format!("{}{}{} {}", &record.candidate_name_first, if record.candidate_name_middle.is_empty() { "" } else { " " }, &record.candidate_name_middle, &record.candidate_name_last);
        let candidate_idx = *candidates_ids.get(&name).unwrap();
        let district: &mut District = districts.get_mut(&record.district_id).unwrap();
        if !district.candidates.contains(&candidate_idx) {
            district.candidates.insert(candidate_idx);
        }
    }

    let mut districts_results: HashMap<DistrictID, DistrictResults> = HashMap::with_capacity(districts.len());
    for (&id, district) in districts.iter() {
        let mut votes: HashMap<CandidateID, u32> = HashMap::new();
        for candidate in district.candidates.iter() {
            votes.insert(*candidate, candidates_votes[candidates[candidate].name.as_ref().unwrap().as_str()]);
        }
        districts_results.insert(id, DistrictResults { candidate_votes: votes, party_list_source: PartyListSource::District, party_votes: HashMap::new() });
    }
    
    let mut areas: HashMap<AreaID, Area> = [
            (10, "Newfoundland and Labrador"),
            (11, "Prince Edward Island"),
            (12, "Nova Scotia"), 
            (13, "New Brunswick"),
            (24, "Quebec"),
            (35, "Ontario"),
            (46, "Manitoba"),
            (47, "Saskatchewan"),
            (48, "Alberta"),
            (59, "British Columbia"),
            (60, "Yukon"),
            (61, "Northwest Territories"),
            (62, "Nunavut")].into_iter().map(|(id, name)| 
                (*id, Area {
                    name: name.to_string(),
                    districts: HashSet::new(),
                    candidates: HashSet::with_capacity(0)
                })).collect();

    for (&district_id, district) in districts.iter() {
        let area = areas.get_mut(&district.area).unwrap();
        area.districts.insert(district_id);
    }

    let mut groupings: HashMap<u32, Grouping> = HashMap::new();
    groupings.insert(1u32, Grouping(districts.keys().map(|&i| {let mut h = HashSet::with_capacity(1); h.insert(i); h }).collect()));
    // TODO: groupings for 2

    return Ok((
        ElectionStage {
            districts,
            areas,
            candidates,
            parties,
        },
        ElectionResults {
            districts: districts_results,
            date
        },
        groupings,
    ))
}