extern crate vsim2;
use vsim2::core::*;

use crate::ui::*;
use std::sync::{Arc,Weak};
use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct Props {
    pub stage: Weak<ElectionStage>,
    pub results: Weak<ElectionResults>,
    pub district: Option<DistrictID>,
}

impl PartialEq for Props {
    fn eq(&self, other: &Props) -> bool {
        self.stage.ptr_eq(&other.stage) && self.results.ptr_eq(&other.results) && self.district == other.district
    }
}

#[derive(Clone)]
pub struct Info {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for Info {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props, link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let stage = self.props.stage.upgrade().unwrap();
        let results = self.props.results.upgrade();

        if let Some(id) = self.props.district {
            let district = &stage.districts[&id];
            html!(
                <div class="dinfo"> 
                    <h3>{&district.name}</h3>
                    <>
                        {{
                            let mut candidates: Vec<CandidateID> = district.candidates.iter().map(|&x| x).collect();
                            if let Some(results) = &results {
                                candidates.sort_by(|&a, &b|
                                    results.districts[&id].candidate_votes[&b].cmp(&results.districts[&id].candidate_votes[&a])
                                );
                            } else {
                                candidates.sort();
                            }
                            candidates.iter().map(|&cid| {
                                html!(<div class="dinfo-candidate">
                                    <span class="name">{stage.candidates[&cid].name.as_ref().map(String::clone).unwrap_or(format!("Candidate {}", cid))}</span>
                                    <span class="party" style={format!("color:{}", color_to_hex(
                                        stage.candidates[&cid].party.map(|party| stage.parties[&party].color).unwrap_or(0xaaaaaa)
                                    ))}>
                                        {stage.candidates[&cid].party.map(|party| stage.parties[&party].name.as_str()).unwrap_or("Independent")}</span>
                                    <>{if let Some(results) = &results {
                                        html!(<span class="votes">{results.districts[&id].candidate_votes[&cid]}</span>)
                                    } else {"".into()}}</>
                                </div>)
                            }).collect::<Html>()
                        }}
                    </>
                    <>
                        {{
                            let district_results = &results.as_ref().unwrap().districts[&id];
                            let mut parties: Vec<_> = district_results.party_votes.keys().map(|&k|k ).collect();
                            parties.sort_by(|&a, &b| {
                                district_results.party_votes[&b].cmp(&district_results.party_votes[&a])
                            });
                            if results.is_some() && !results.as_ref().unwrap().districts[&id].party_votes.is_empty() {
                                html!(<>
                                    <h5>{"List Votes"}</h5>
                                    {{
                                        parties.iter().map(|&party_id| {
                                            let votes = district_results.party_votes[&party_id];
                                            html!(<div class="dinfo-list-votes">
                                                <span class="party" style={format!("color:{}",color_to_hex(stage.parties[&party_id].color))}>{&stage.parties[&party_id].name}</span>
                                                <span class="votes">{votes}</span>
                                            </div>)
                                        }).collect::<Html>()
                                    }}
                                </>)
                            } else {
                                "".into()
                            }
                        }}
                    </>
                </div>
            )
        } else {
            html!(<div class="dinfo"></div>)
        }
    }
}