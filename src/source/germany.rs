use crate::core::*;
use std::collections::{HashMap, HashSet};
use std::{io,fmt};
use std::borrow::Cow;

pub enum Error {
    Custom(Cow<'static, str>),
    Parse(std::num::ParseIntError, Cow<'static, str>),
    Csv(csv::Error)
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Csv(csv) => {
                write!(f, "germany parsing error: csv error: {}", csv)
            },
            Error::Custom(msg) => {
                write!(f, "germany parsing error: {}", msg.as_ref())
            },
            Error::Parse(err, msg) => {
                write!(f, "germany parsing error: {}: {}", msg, err)
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Csv(csv) => {
                Some(csv)
            },
            _ => None
        }
    }
}

impl From<csv::Error> for Error {
    fn from(e: csv::Error) -> Self {
        Error::Csv(e)
    }
}

pub fn from_csv(reader: impl io::Read + io::Seek) -> Result<(ElectionStage, ElectionResults, HashMap<u32, Grouping>), Error> {
    let mut r = csv::ReaderBuilder::new().delimiter(b';').has_headers(false).flexible(true).from_reader(reader);

    #[derive(Default)]
    struct Positions {
        pub id: usize,
        pub name: usize,
        pub parent: usize,
        pub population: usize,
        pub parties: HashMap<PartyID, usize>,
    }

    let mut pos: Positions = Default::default();
    let mut date: Option<Date> = None;

    let mut parties = HashMap::new();
    let mut areas_pop = HashMap::new();
    let mut areas = HashMap::new();
    let mut districts = HashMap::new();
    let mut district_results = HashMap::new();
    let mut candidates = HashMap::new();

    let mut candidate_id: CandidateID = 0;
    for record  in r.records() {
        let record = record?;
        if record.len() == 0 {
            Err(Error::Custom("Record cannot have length 0".into()))?;
        }

        if record.get(0).unwrap().starts_with("#") {
            if let None = date {
                let last = record.get(0).unwrap().split(' ').rev().next().unwrap();
                if let Ok(res) = last.parse::<u32>() {
                    date = Some(Date::new(res, 1, 1));
                } else {
                    dbg!(last);
                }
            }

            continue
        }

        if record.get(0).unwrap() == "Nr" {
            for (i, value) in record.iter().enumerate() {
                match value {
                    "Nr" => pos.id = i,
                    "Gebiet" => pos.name = i,
                    "gehört zu" => pos.parent = i,
                    "Wahlberechtigte" => pos.population = i,
                    "Wähler" | "Ungültige" | "Gültige" => continue,
                    "" => continue,
                    _ => {
                        let party_data = match value {
                            "Christlich Demokratische Union Deutschlands" => ("Christian Democratic Union", 0x322f2e, PartyType::Conservative),
                            "Sozialdemokratische Partei Deutschlands" => ("Social Democrats", 0xe2001a, PartyType::SocialDemocratic),
                            "DIE LINKE" => ("Linke", 0xbe3075, PartyType::Left),
                            "BÜNDNIS 90/DIE GRÜNEN" => ("Green", 0x19a329, PartyType::Green),
                            "Christlich-Soziale Union in Bayern e.V." => ("Christian Social Union", 0x008bc6, PartyType::Conservative),
                            "Freie Demokratische Partei" => ("Free Democrats", 0xffee00, PartyType::Liberal),
                            "Alternative für Deutschland" => ("Alternative for Germany", 0x00a0e2, PartyType::Fascist),
                            _ => (value, 0xaaaaaa, PartyType::Other)
                        };

                        parties.insert((i / 4) as PartyID, Party {
                            name: party_data.0.to_owned(),
                            type_: party_data.2,
                            color: party_data.1,
                        });
                        pos.parties.insert((i / 4) as PartyID, i);
                    }
                };
            }
            continue
        }

        if record.get(0).unwrap().len() == 0 { 
            continue
        }

        let id = record.get(pos.id).ok_or(Error::Custom("id col not found".into()))?.parse::<DistrictID>().map_err(|err| Error::Parse(err, "id could not be parsed".into()))?;
        let name = record.get(pos.name).ok_or(Error::Custom("name col not present".into()))?.to_owned();
        let parent = match record.get(pos.parent).ok_or(Error::Custom("parent col not present".into()))?.parse::<AreaID>() {
            Ok(v) => v,
            Err(e) => continue
        };
        if parent == 99 {
            areas.insert(id as AreaID, Area {
                name: name,
                districts: HashSet::new(),
                candidates: HashSet::new(),
                seats: 0
            });
            let pop = record.get(pos.population).ok_or(Error::Custom("Wahlberechtigte col not present".into()))?.parse::<u32>().map_err(|err| Error::Parse(err, "population couldn't be parsed".into()))?;
            areas_pop.insert(id as AreaID, pop);
        } else {
            let mut district = District {
                name,
                area: parent,
                seats: 1,
                candidates: HashSet::new(),
            };

            let mut results = DistrictResults {
                party_votes: HashMap::new(),
                party_list_source: PartyListSource::Area,
                candidate_votes: HashMap::new(),
            };

            // Local candidates
            for (&party, &pos) in pos.parties.iter() {
                let first_vote = match record.get(pos).ok_or(Error::Custom(("party col 1st vote ".to_owned() + &parties[&party].name + " not present").into()))?.parse::<u32>() {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                let cid = candidate_id;
                candidate_id += 1;
                candidates.insert(cid, Candidate {
                    name: None,
                    party: Some(party),
                });
                district.candidates.insert(cid);

                results.candidate_votes.insert(cid, first_vote);
            }

            // List candidates
            for (&party, &pos) in pos.parties.iter() {
                let second_vote = match record.get(pos+2).ok_or(Error::Custom(("party col 2nd vote ".to_owned() +  &parties[&party].name + " not present").into()))?.parse::<u32>() {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                results.party_votes.insert(party, second_vote);
            }

            districts.insert(id as DistrictID, district);
            district_results.insert(id as DistrictID, results);
        }
    }

    // Add `districts` to areas
    for (&id, district) in districts.iter() {
        areas.get_mut(&district.area).unwrap().districts.insert(id);

        for (&party, _) in &district_results[&id].party_votes {
            // Assume 1 candidate for a party for each district where the party recieved votes.
            let cid = candidate_id;
            candidate_id += 1;
            candidates.insert(cid, Candidate { name: None, party: Some(party) });

            areas.get_mut(&district.area).unwrap().candidates.insert(cid);
        }
    }

    let mut groupings = HashMap::new();
    groupings.insert(1u32, Grouping(districts.keys().map(|&i| {let mut h = HashSet::with_capacity(1); h.insert(i); h }).collect()));

    {
        dbg!((districts.len() * 2) as SeatCount);
        let areas_sorted = areas.keys().map(|&x| x).collect::<Vec<_>>();
        let areas_pop_vec = areas_sorted.iter().map(|&area| areas_pop[&area]).collect::<Vec<_>>();
        let areas_seats = utils::allocate_sainte_lague(&areas_pop_vec, (districts.len() * 2) as SeatCount);
        for (idx, &seats) in areas_seats.iter().enumerate() {
            let local_seats = areas[&areas_sorted[idx]].districts.len() as SeatCount;
            areas.get_mut(&areas_sorted[idx]).unwrap().seats = seats - local_seats;
        }
    }

    return Ok((
        ElectionStage {
            candidates, areas, districts, parties
        },  
        ElectionResults {
            date: date.unwrap(),
            districts: district_results
        },
        groupings
    ))
}