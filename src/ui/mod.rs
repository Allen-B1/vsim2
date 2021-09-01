use crate::*;
use yew::prelude::*;

#[derive(Clone)]
pub struct Model {
    link: ComponentLink<Self>,
    stage: Option<ElectionStage>,
    results: Option<ElectionResults>,
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
            stage: Some(serde_json::from_slice(stage_bytes).unwrap()),
            results: Some(serde_json::from_slice(results_bytes).unwrap()),
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
                            <div class="map">
                                { for stage.areas.iter().map(|area| html!(<div class="area"></div>))}
                            </div>
                        )
                    } else {
                        html!(
                            <div class="status">{"Loading"}</div>
                        )
                    }
                }
            </main>
        )
    }
}