use crate::*;
use std::sync::{Arc,Weak};
use yew::prelude::*;

mod district_info;
mod map;

#[derive(Clone)]
pub struct Model {
    link: ComponentLink<Self>,
    stage: Option<Arc<ElectionStage>>,
    results: Option<Arc<ElectionResults>>,
    groupings: Option<Arc<Groupings>>,
    district: Option<DistrictID>,
}

#[derive(Clone)]
pub enum Msg {
    SelectDistrict(DistrictID)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let (stage, results, groupings) = core::decode(&mut &include_bytes!("../../dataset/canada-2019/data.elc")[..]).unwrap();
        
        Model {
            link,
            stage: Some(Arc::new(stage)),
            results: Some(Arc::new(results)),
            groupings: Some(Arc::new(groupings)),
            district: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SelectDistrict(district) => {
                self.district = Some(district);
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