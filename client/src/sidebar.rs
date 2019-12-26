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
            Link { text: "log workout".to_string(), url: "/log_workout".to_string() },
            Link { text: "log abs".to_string(), url: "/log_abs".to_string() },
            Link { text: "".to_string(), url: "/".to_string() },
            Link { text: "statistics workout".to_string(), url: "/st_workout".to_string() },
            Link { text: "statistics abs".to_string(), url: "/st_abs".to_string() },
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
        let _links = &self.state.links
            .iter()
            .enumerate()
            .map(|x| self.view_links(x));

        html! {
            <div class="col sidebar" onclick=|_| Msg::UpdateTitle>
                { for self.state.links.iter().enumerate().map(|x|self.view_links(x)) }
            </div>
        }
    }
}

impl Sidebar {
    fn view_links(&self, (_idx, template): (usize, &Link)) -> Html<Self> {
        html! {
            <SideBarLink text={&template.text} 
                         to={&template.url} />
        }
    }
}