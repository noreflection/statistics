use yew::prelude::*;

use crate::header::ListHeader;
use crate::sidebar::Sidebar;
use crate::sidebar_link::SideBarLink;
use crate::item::ListItem;
use crate::list::{List, Msg as ListMsg};

use yew::html::{ChildrenRenderer, NodeRef};

pub struct Body {
    title: String,
}

//#[derive(Properties)]
//pub struct Props {
//    #[props(required)]
//    pub children: ChildrenRenderer<SideBarLink>,
//}

pub enum Msg {
    UpdateTitle,
}

impl Component for Body {
    type Message = Msg;
    type Properties = ();

    fn create(_: (), _: ComponentLink<Self>) -> Self {
        Body { title: String::from("index") }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateTitle => {
                self.title = String::from("updated");
                true
            }
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="col other">
                <List>
                    <ListHeader text="Calling all Rusties!" on_hover=ListMsg::Hover />
                    <ListItem name="Rustin" on_hover=ListMsg::Hover />
                    <ListItem hide={true} name="Rustaroo" on_hover=ListMsg::Hover />
                    <ListItem name="Rustifer" on_hover=ListMsg::Hover>
                        <span>{"Hello!"}</span>
                    </ListItem>
                </List>
            </div>
        }
    }
}
