use vsim2::core::*;
use vsim2::*;
use yew::prelude::*;

use super::Model;

#[derive(Clone)]
pub struct MethodPicker {
    link: ComponentLink<Self>,
    root: ComponentLink<Model>,
    method: Option<Box<dyn ElectoralMethod>>,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub root: ComponentLink<Model>,
}

impl Component for MethodPicker {
    type Message = Box<dyn ElectoralMethod>;
    type Properties = Props;

    fn view(&self) -> Html {
        html!(
            <div class="method-picker">
                <button
                    class={format!("{}", if self.method.is_some() && self.method.as_ref().unwrap().as_any().is::<methods::fptp::FPTP>() { "active" } else { "" } )}
                    onclick=self.link.callback(|_| Box::new(methods::fptp::FPTP) as Box<dyn ElectoralMethod>)>{"FPTP"}</button>
                <button
                    class={format!("{}", if self.method.is_some() && self.method.as_ref().unwrap().as_any().is::<methods::dmp::DMP>() { "active" } else { "" } )}
                    onclick=self.link.callback(|_| Box::new(methods::dmp::DMP { threshold : 0.05 }) as Box<dyn ElectoralMethod>)>{"DMP"}</button>
            </div>
        )
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.method = Some(msg.clone());
        self.root.send_message(super::Msg::ElectoralMethod(msg));
        true
    }

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        MethodPicker {
            link,
            root: props.root,
            method: None
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}