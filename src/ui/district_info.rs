use crate::*;
use crate::ui::*;
use std::sync::{Arc,Weak};
use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct Props {
    pub stage: Weak<ElectionStage>,
    pub results: Weak<ElectionResults>,
    pub district: Option<usize>,
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
            let district = &stage.districts[id];
            html!(
                <div class="dinfo"> 
                    <h5>{&district.name}</h5>
                    {{
                        let mut candidates: Vec<usize> = district.candidates.iter().map(|&x| x).collect();
                        if let Some(results) = &results {
                            candidates.sort_by(|a, b|
                                results.results[id].votes[&b].cmp(&results.results[id].votes[&a])
                            );
                        } else {
                            candidates.sort();
                        }
                        candidates.iter().map(|&cid| {
                            html!(<div class="dinfo-candidate">
                                <span class="name">{&stage.candidates[cid].name}</span>
                                <span class="party" style={format!("color:{}", color_to_hex(
                                    stage.candidates[cid].party.map(|party| stage.parties[party].color).unwrap_or(0xaaaaaa)
                                ))}>
                                    {stage.candidates[cid].party.map(|party| stage.parties[party].name.as_str()).unwrap_or("Independent")}</span>
                                <>{if let Some(results) = &results {
                                    html!(<span class="votes">{results.results[id].votes[&cid]}</span>)
                                } else {"".into()}}</>
                            </div>)
                        }).collect::<Html>()
                    }}
                </div>
            )
        } else {
            html!(<div class="dinfo"></div>)
        }
    }
}