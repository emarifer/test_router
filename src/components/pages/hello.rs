use yew::prelude::{function_component, html, Callback, Html};
use yew_router::prelude::use_navigator;

use crate::router::Route;

#[function_component(Hello)]
pub fn hello() -> Html {
    let navigator = use_navigator().unwrap();
    let go_home_button = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Home))
    };

    html! {
      <div class="content-style">
        <h1 style={"display:inline;margin:0"}>{"Hello"}</h1>
        <button onclick={go_home_button}>{"Go Home"}</button>
      </div>
    }
}
