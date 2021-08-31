use std::io;
use serde::{Deserialize};
use std::collections::{HashMap,HashSet};

#[derive(Deserialize)]
struct PollRecord {
    #[serde(rename = "Electoral District Number/Numéro de circonscription")]
    district_id: u32,

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

pub fn from_zip(reader: impl io::Read + io::Seek) -> Result<(crate::ElectionStage, crate::ElectionResults), Box<dyn std::error::Error>> {
    let mut archive = zip::ZipArchive::new(reader)?; 
    let mut records: Vec<PollRecord> = Vec::new();
    for i in 0..archive.len() {
        let csv_file = archive.by_index(i)?;

        let mut rdr = csv::Reader::from_reader(csv_file);
        for result in rdr.deserialize::<PollRecord>() {
            records.push(result?);
        }
    }
    let mut parties: HashMap<String, crate::Party> = HashMap::new();
    for record in records.iter() {
        if let None = parties.get(&record.party) {
            let data = match record.party.as_str() {
                "Conservative" => crate::Party {
                    name: "Conservative".to_string(),
                    color: 0x14294d,
                    type_: crate::PartyType::Conservative,
                },
                "Liberal" => crate::Party {
                    name: "Liberal".to_string(),
                    color: 0xda121a,
                    type_: crate::PartyType::Liberal,
                },
                "NDP-New Democratic Party" => crate::Party {
                    name: "NDP".to_string(),
                    color: 0xef7c00,
                    type_: crate::PartyType::SocialDemocratic,
                },
                "Bloc Québécois" => crate::Party {
                    name: "Bloc Québécois".to_string(),
                    color: 0x42b7bf,
                    type_: crate::PartyType::Other,
                },
                "Green Party" => crate::Party {
                    name: "Green Party".to_string(),
                    color: 0x3d9b35,
                    type_: crate::PartyType::Green,
                },
                "Independent" => continue,
                _ => crate::Party {
                    name: record.party.to_string(),
                    color: 0xaaaaaa,
                    type_: crate::PartyType::Other,
                }
            };
            parties.insert(record.party.to_string(), data);
        }
    }

    let parties_order = parties.keys().map(Clone::clone).collect::<Vec<String>>();
    let parties_idx = parties_order.iter().enumerate().map(|(i, v)| (v.clone(), i)).collect::<HashMap<String, usize>>();
    let parties_vec = parties_order.iter().map(|name| parties[name].clone()).collect::<Vec<crate::Party>>();

    let mut candidates_map: HashMap<String, crate::Candidate> = HashMap::new();
    let mut candidates_votes: HashMap<String, u32> = HashMap::new();
    for record in records.iter() {
        let name = format!("{}{}{} {}", &record.candidate_name_first, if record.candidate_name_middle.is_empty() { "" } else { " " }, &record.candidate_name_middle, &record.candidate_name_last);

        if let None = candidates_map.get(&name) {
            candidates_map.insert(name.clone(), crate::Candidate {
                name: name.clone(),
                party: parties_idx.get(&record.party).map(|r| *r),
            });
        }

        *candidates_votes.entry(name).or_insert(0) += record.votes;
    }
    let candidates_order = candidates_map.keys().map(Clone::clone).collect::<Vec<String>>();
    let candidates_vec = candidates_order.iter().map(|name| candidates_map[name.as_str()].clone()).collect::<Vec<crate::Candidate>>();
    let candidates_idx = candidates_order.iter().enumerate().map(|(i, v)| (v.clone(), i)).collect::<HashMap<String, usize>>();

    let mut districts_map: HashMap<u32, crate::District> = HashMap::new();
    for record in records.iter() {
        if let None = districts_map.get(&record.district_id) {
            districts_map.insert(record.district_id, crate::District {
                name: record.district_name.clone(),
                candidates: HashSet::new(),
                size: 1,                
            });
        }

        let name = format!("{}{}{} {}", &record.candidate_name_first, if record.candidate_name_middle.is_empty() { "" } else { " " }, &record.candidate_name_middle, &record.candidate_name_last);
        let candidate_idx = *candidates_idx.get(&name).unwrap();
        let district: &mut crate::District = districts_map.get_mut(&record.district_id).unwrap();
        if !district.candidates.contains(&candidate_idx) {
            district.candidates.insert(candidate_idx);
        }
    }
    let districts_order = districts_map.keys().map(Clone::clone).collect::<Vec<u32>>();
    let districts_vec = districts_order.iter().map(|name| districts_map[&name].clone()).collect::<Vec<crate::District>>();
    let districts_idx = districts_order.iter().enumerate().map(|(i, v)| (*v, i)).collect::<HashMap<u32, usize>>();

    let mut districts_results: Vec<crate::DistrictResult> = Vec::with_capacity(districts_vec.len());
    for district in districts_vec.iter() {
        let mut votes: HashMap<usize, u32> = HashMap::new();
        for candidate in district.candidates.iter() {
            votes.insert(*candidate, candidates_votes[&candidates_order[*candidate]]);
        }
        districts_results.push(crate::DistrictResult { votes });
    }
    
    let mut areas_vec = ["Newfoundland and Labrador", "Prince Edward Island", "Nova Scotia", "New Brunswick", "Quebec", "Ontario",
            "Manitoba",
            "Saskatchewan",
            "Alberta",
            "British Columbia",
            "Yukon",
            "Northwest Territories",
            "Nunavut"].iter().map(|name| 
                crate::Area {
                    name: name.to_string(),
                    districts: HashSet::new(),
                }).collect::<Vec<crate::Area>>();

    use std::iter::{FromIterator};
    use std::array::IntoIter;
    let areas_idx: HashMap<u32, usize> = HashMap::<u32,usize>::from_iter(IntoIter::new([
        (10, 0),
        (11, 1),
        (12, 2),
        (13, 3),
        (24, 4),
        (35, 5),
        (46, 6),
        (47, 7),
        (48, 8),
        (59, 9),
        (60, 10),
        (61, 11),
        (62, 12)
    ]));

    for record in records.iter() {
        let area_id = record.district_id / 1000;
        let area = areas_vec.get_mut(areas_idx[&area_id]).unwrap();
        area.districts.insert(*districts_idx.get(&record.district_id).unwrap());
    }

    return Ok((
        crate::ElectionStage {
            districts: districts_vec,
            areas: areas_vec,
            candidates: candidates_vec,
            parties: parties_vec,
        },
        crate::ElectionResults {
            results: districts_results,
            date: crate::Date { year: 2019, month: 1, day: 1 }
        }
    ))
}