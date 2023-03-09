use crate::router::Route;
use yew::prelude::{function_component, html, Html};
use yew_router::prelude::Link;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
      <div class="content-style">
        <h1 style={"display:inline;margin:0"}>{"Home"}</h1>
        <Link<Route> to={Route::Hello}>{ "To Hello" }</Link<Route>>
      </div>
    }
}
