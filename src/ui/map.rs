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
    pub root: ComponentLink<Model>, // TODO: is this a reference cycle?
}

impl PartialEq for Props {
    fn eq(&self, other: &Props) -> bool {
        self.stage.ptr_eq(&other.stage) && self.results.ptr_eq(&other.results) && self.district == other.district
    }
}

#[derive(Clone)]
pub  struct Map {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for Map {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props, link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.props.root.send_message(msg);
        
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
        let stage= self.props.stage.upgrade().unwrap();
        let results = self.props.results.upgrade();
        html!(
            <div class="map">
                {{
                    let mut areas: Vec<u16> = stage.areas.keys().map(|&x| x).collect();
                    areas.sort();
                    areas.iter().map(|&area_id| {
                        let area = &stage.areas[&area_id];
                        html!(
                            <div class="map-area">
                                <h5 class="map-area-name">{&area.name}</h5>
                                <div class="map-area-districts" style={format!("width: {}px", (32+8) * ((area.districts.len() as f64).sqrt().ceil() as u32))}>
                                    {
                                        {
                                            let mut districts: Vec<DistrictID> = area.districts.iter().map(|&x| x).collect();
                                            districts.sort();
                                            districts.iter().map(|&id| (id, &stage.districts[&id])).map(|(id, district)| {
                                                let mut classes = classes!("map-district");
                                                if Some(id) == self.props.district {
                                                    classes.push("selected");
                                                }
                                                
                                                html!(<button class=classes onclick=self.link.callback(move |_| Msg::SelectDistrict(id)) style={
                                                    if let Some(results) = &results { "background:".to_string() + &color_to_hex(results.districts[&id].candidate_votes.iter()
                                                        .reduce(|c1, c2| if c1.1 > c2.1 { c1 } else { c2 })
                                                        .and_then(|(&c, _)| stage.candidates[&c].party)
                                                        .map(|party| stage.parties[&party].color)
                                                        .unwrap_or(0xaaaaaa)) } else { "".to_string() }
                                                }>{abbr(&district.name)}</button>)
                                            }).collect::<Html>()
                                        }
                                    }
                                </div>
                            </div>
                        )
                    }).collect::<Html>()
                }}
            </div>
        )        
    }
}

fn abbr(name: &str) -> String {
    let items: Vec<&str> = name.split(" ").collect();
    if items.len() != 1 {
        let chars: Vec<char> = items.iter().map(|x| x.chars().next().unwrap_or(' ')).collect();
        if chars.len() <= 3 { chars.into_iter().collect() }
        else {
            chars.iter().filter(|x| x.is_uppercase()).collect()
        }
    } else {
        name.chars().take(2).collect()
    }
}