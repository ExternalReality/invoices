#![recursion_limit = "512"]

#[macro_use]
extern crate yew;

use yew::prelude::*;

struct Model {
    value: String,
}

enum Msg {
    DoIt,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            value: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => self.value = String::from("Thank you for reviewing this for me!"),
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="container",>
                <div id="company_name",>
                   {"["}
                   <input type="text", placeholder="Company Name",/>
                   {"]"}
                </div>
                <div id="address",>
                    <div id="street",>
                       <span>{"["}</span>
                       <input type="text", placeholder="Street Address",/>
                       <span>{"]"}</span>
                    </div>
                    <div id="city-st-zip",>
                       <span>{"["}</span>
                       <input type="text", placeholder="City, State, Zip",/>
                       <span>{"]"}</span>
                    </div>
                    <div id="phone",>
                       <label>{"Phone:"}</label>
                       <input type="text", placeholder="(000)-000-0000",/>
                    </div>
                 </div>
                <div id="banner",>{"invoice"}</div>
                    <div id="inv-banner-container",>
                        <div id="inv-banner",>
                        <span>{"invoice #"}</span>
                        <span>{"date"}</span>
                    </div>
                <div id="inv-banner-info",>
                  <span id="inv_date",>{ "04/24/2020" }</span>
                    </div>
                </div>
                <div id="customer-id-container",>
                <div id="cust-banner",>
                <span>{"Customer ID"}</span>
                <span>{"Terms"}</span>
                </div>
                <div id="customer-id-info",>
                <span id="customer-id",>{ "773438" }</span>
                <span id="Terms",>{ "Due Upon Receipt" }</span>
                </div>
                </div>

                <div id="bill-to-container",>
                <div id="bill-to-banner",>
                <span>{"Bill To"}</span>
                </div>
                <div id="bill-to-banner-info",>
                </div>
                </div>

            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
