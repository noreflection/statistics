use yew::prelude::*;
use crate::sidebar_link::SideBarLink;

pub struct Sidebar {
    state: State,
}

pub struct State {
    title: String,
    links: Vec<Link>,
}

struct Link {
    text: String,
    url: String,
}

pub enum Msg {
    UpdateTitle,
}

impl Component for Sidebar {
    type Message = Msg;
    type Properties = ();

    fn create(_: (), _: ComponentLink<Self>) -> Self {
        let links = vec![
            Link { text: "first_link".to_string(), url: "first_url".to_string() },
            Link { text: "second_link".to_string(), url: "second_url".to_string() },
        ];
        let state = State {
            title: "initial_title".to_string(),
            links,
        };

        Sidebar { state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateTitle => {
                self.state.title = String::from("updated_title");
                true
            }
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="col sidebar" onclick=|_| Msg::UpdateTitle>
                { &self.state.title }
                <SideBarLink text="first" />
                <SideBarLink text="second" />
            </div>
        }
    }
}