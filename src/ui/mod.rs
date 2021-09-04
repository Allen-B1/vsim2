extern crate vsim2;
use vsim2::core::*;
use std::{any::Any, sync::{Arc,Weak}};
use yew::prelude::*;
use vsim2::core;

mod district_info;
mod map;
mod parliament;
mod method_picker;

#[derive(Clone)]
pub struct Model {
    link: ComponentLink<Self>,
    stage: Option<Arc<ElectionStage>>,
    results: Option<Arc<ElectionResults>>,
    groupings: Option<Arc<Groupings>>,
    district: Option<DistrictID>,
    
    seats: Option<Arc<SeatResult>>
}

#[derive(Clone)]
pub enum Msg {
    SelectDistrict(DistrictID),
    ElectoralMethod(Box<dyn ElectoralMethod>)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let (stage, results, groupings) = core::decode(&mut &include_bytes!("../../dataset/germany-2017/data.elc")[..]).unwrap();
        
        Model {
            link,
            stage: Some(Arc::new(stage)),
            results: Some(Arc::new(results)),
            groupings: Some(Arc::new(groupings)),
            seats: None,
            district: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SelectDistrict(district) => {
                self.district = Some(district);
                true
            },
            Msg::ElectoralMethod(method) => {
                if let (Some(stage), Some(results), Some(groupings)) = (&self.stage, &self.results, &self.groupings) {
                    let grouping = groupings.get(&method.district_size()).unwrap_or(groupings.iter().next().unwrap().1);
                    match method.run(stage.as_ref(), results.as_ref(), grouping) {
                        Ok(val) => self.seats = Some(Arc::new(val)),
                        Err(e) => eprintln!("{}", e),
                    }
                }
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender { 
        false
    }

    fn view(&self) -> Html {
        html!(
            <main>
                {
                    if let Some(stage) = &self.stage {
                        html!(
                            <>
                                <method_picker::MethodPicker root=self.link.clone()></method_picker::MethodPicker>
                                <div class="parliament-container">{
                                    if let Some(seats) = &self.seats {
                                        let mut seats: Vec<_> = utils::seats_by_party(stage, seats).iter().map(|(&partyopt, &count)| {
                                            (partyopt, count)
                                        }).collect();
                                        seats.sort_by(|(a, _), (b, _)| {
                                            let ord = a.and_then(|x| stage.parties.get(&x)).map(|p| p.type_).unwrap_or_default().cmp(&b.and_then(|x| stage.parties.get(&x)).map(|p| p.type_).unwrap_or_default());
                                            if ord != std::cmp::Ordering::Equal {
                                                return ord;
                                            }

                                            a.unwrap_or(PartyID::MAX).cmp(&b.unwrap_or(PartyID::MAX))
                                        });
                                        parliament::generate(seats.iter().map(|(partyopt, count)| {
                                            match partyopt {
                                                Some(pid) => 
                                                    (stage.parties[&pid].name.as_str(), *count as u32, stage.parties[&pid].color),
                                                None => ("Independent", *count as u32, 0xaaaaaa)
                                            }
                                        }))
                                    } else {
                                        "".into()
                                    }
                                }</div>
                                <map::Map
                                    stage=Arc::downgrade(stage) results=self.results.as_ref().map(Arc::downgrade).unwrap_or(Weak::new()) district=self.district
                                    root=self.link.clone()></map::Map>
                                <district_info::Info
                                    stage=Arc::downgrade(stage) results=self.results.as_ref().map(Arc::downgrade).unwrap_or(Weak::new()) district=self.district>
                                </district_info::Info>
                            </>
                        )
                    } else {
                        html!(<div class="status">{"Loading..."}</div>)
                    }
                }
            </main>
        )
    }
}


fn color_to_hex(clr: u32) -> String {
    format!("#{:06x}", clr & 0xffffff)
}