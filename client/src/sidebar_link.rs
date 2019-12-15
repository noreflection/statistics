use yew::prelude::*;

pub struct SideBarLink {
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub text: String,
    pub to: String,
}

pub enum Msg {
    UpdateText,
}

impl Component for SideBarLink {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        SideBarLink { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateText => {
                true
            }
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="sidebar__link" onclick=|_| Msg::UpdateText href={String::from(&self.props.to)}>
                { &self.props.text }
            </div>
        }
    }
}