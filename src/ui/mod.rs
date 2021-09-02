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
    district: Option<usize>,
}

#[derive(Clone)]
pub enum Msg {
    SelectDistrict(usize)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let stage_bytes = include_bytes!("../../dataset/canada-2019/stage.json");
        let results_bytes = include_bytes!("../../dataset/canada-2019/results.json");

        
        Model {
            link,
            stage: Some(Arc::new(serde_json::from_slice(stage_bytes).unwrap())),
            results: Some(Arc::new(serde_json::from_slice(results_bytes).unwrap())),
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