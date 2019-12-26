#![recursion_limit = "512"]

#[allow(dead_code)]
use yew::prelude::*;

mod sidebar;
mod sidebar_link;
mod body;
mod card;

use sidebar::Sidebar;
use body::Body;

pub struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="main">
                    <div class="app_container">
                            <Sidebar />
                            <Body />
                    </div>
            </div>
        }
    }
}
