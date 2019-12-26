use yew::prelude::*;
//use yew::html::{ChildrenRenderer, NodeRef};

use crate::card::Card;

pub struct Body {
    title: String,
}

impl Component for Body {
    type Message = ();
    type Properties = ();

    fn create(_: (), _: ComponentLink<Self>) -> Self {
        Body {
            title: String::from("log workout")
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            _ => true
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="col other card_container">
//                {&self.title}
                <div class="card">
                    <Card title={"Barbell Bench Press"}
                          phase={"first phase"} />
                 </div>
                
                <div class="card">
                    <Card title={"Barbell Squat"}
                          phase={"first phase"} />
                 </div>
                 
                <div class="card">
                    <Card title={"Barbell Squat"}
                          phase={"first phase"} />
                 </div>
                 
                <div class="card">
                    <Card title={"Barbell Squat"}
                          phase={"first phase"} />
                 </div>
                 
                <div class="card">
                    <Card title={"Barbell Squat"}
                          phase={"first phase"} />
                 </div>
                 
                <div class="card">
                    <Card title={"Barbell Squat"}
                          phase={"first phase"} />
                 </div>
                 
                <div class="card">
                    <Card title={"Barbell Squat"}
                          phase={"first phase"} />
                 </div>
                 
                <div class="card">
                    <Card title={"Barbell Squat"}
                          phase={"first phase"} />
                 </div>
                 
                <div class="card">
                    <Card title={"Barbell Squat"}
                          phase={"first phase"} />
                 </div>
                 
                <div class="card">
                    <Card title={"Barbell Squat"}
                          phase={"first phase"} />
                 </div>
                 
                <div class="card">
                    <Card title={"Barbell Squat"}
                          phase={"first phase"} />
                 </div>
                 
                <div class="card">
                    <Card title={"Barbell Squat"}
                          phase={"first phase"} />
                 </div>
            </div>
        }
    }
}
