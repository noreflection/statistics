use yew::prelude::*;

pub struct SidebarLink {
    //props: Props,
    inner_text: String,
}

//#[derive(Properties)]
//pub struct Props {
//    #[props(required)]
//    pub text: String,
//}

pub enum Msg {
    UpdateText,
}

impl Component for Sidebar {
    type Message = Msg;
    type Properties = ();

    fn create(_: (), _: ComponentLink<Self>) -> Self {
        Sidebar { inner_text: String::from("init") }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateText => {
                self.inner_text = String::from("updated");
                true
            }
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="menu" onclick=|_| Msg::UpdateText>
                { &self.inner_text }
            </div>
        }
    }
}