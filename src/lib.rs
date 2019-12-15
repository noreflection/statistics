#![recursion_limit = "128"]

#[allow(dead_code)]
mod header;
mod item;
mod list;
mod sidebar;
mod sidebar_link;
mod body;

use header::ListHeader;
use sidebar::Sidebar;
use item::ListItem;
use list::{List, Msg as ListMsg};
use body::Body;
use yew::prelude::*;

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
