use yew::prelude::*;

pub struct Card {
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub title: String,
    #[props(required)]
    pub phase: String,
}

impl Component for Card {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Card { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            _ => true
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div class="card__element">
                    <div class="card__header_labels">
                        <div>
                            {&self.props.title}
                        </div>
                
                        <div class="card__phase">
                            {&self.props.phase}
                        </div>
                    </div>
                </div>
                
                <div class="card__element">
                    <div class="card__header_element card__set_weight ">
                        <input type="text" placeholder="set weight" class="input is-small" />
                    </div>
                 </div>
                
                <div class="card__element">
                    <div class="card__header_element card__repetitions select is-small">
                     
                        <select>
                            <option value="first">{"first"}</option>
                            <option value="second">{"second"}</option>
                        </select>
                    </div>
                </div>
                
                <div class="card__element">
                    <div class="card__header_element card__add_weight select is-small">
                        <select>
                            <option value="first">{"first"}</option>
                            <option value="second">{"second"}</option>
                        </select>
                    </div>
                </div>
                
                <div>
                    <button type="button" class="button is-small">{"submit"}</button>
                </div>
            </div>
        }
    }
}