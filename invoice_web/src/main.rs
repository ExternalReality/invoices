#![recursion_limit = "512"]

#[macro_use]
extern crate yew;

use yew::prelude::*;

mod bracketed_input;
use bracketed_input::BracketedInput;

struct Model {}

enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
        <div>
            <div class="columns" ,>
                <div class="column" ,>
                    <div id="company_name" ,>
                        {"["}
                        <input type="text" , placeholder="Company Name" , />
                        {"]"}
                    </div>
                    <div id="address" ,>
                        <BracketedInput: placeholder="Street Address" , />
                        <BracketedInput: placeholder="City, State, Zip" , />
                        <div id="phone" ,>
                            <label>{"Phone:"}</label>
                            <input type="text" , placeholder="(000)-000-0000" , />
                        </div>
                    </div>

                    <div id="bill-to-container" ,>
                        <div id="bill-to-banner" ,>
                            <span>{"Bill To"}</span>
                        </div>
                        <BracketedInput: placeholder="Name" , />
                        <BracketedInput: placeholder="Company Name" , />
                        <BracketedInput: placeholder="Street Address" , />
                        <BracketedInput: placeholder="City, State, Zip" , />
                        <BracketedInput: placeholder="Phone" , />
                        <BracketedInput: placeholder="Email Address" , />
                    </div>
                </div>

                <div class="column" ,>
                    <div id="banner" ,>{"invoice"}</div>
                    <div id="inv-banner-container" ,>
                        <div id="inv-banner" ,>
                            <span>{"invoice #"}</span>
                            <span>{"date"}</span>
                        </div>
                        <div id="inv-banner-info" ,>
                            <span id="inv_date" ,>{ "04/24/2020" }</span>
                        </div>
                    </div>
                    <div id="customer-id-container" ,>
                        <div id="cust-banner" ,>
                            <span>{"Customer ID"}</span>
                            <span>{"Terms"}</span>
                        </div>
                        <div id="customer-id-info" ,>
                            <span id="customer-id" ,>{ "773438" }</span>
                            <span id="Terms" ,>{ "Due Upon Receipt" }</span>
                        </div>
                    </div>
                </div>
            </div>
            <span id="ruler" ,></span>
        </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
