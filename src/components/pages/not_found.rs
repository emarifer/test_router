use yew::prelude::{function_component, html, Callback, Html};
use yew_router::history::{BrowserHistory, History};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let go_back = Callback::from(|_| {
        let history = BrowserHistory::new();
        history.back();
    });

    html! {
      <div class="content-style">
        <h1 style={"display:inline;margin:0;font-size:1rem;color:#4338ca"}>{"Error 404: Not Found"}</h1>
        <button style={"font-size:10px;color:red"} onclick={go_back}>
          {"Go Back"}
        </button>
      </div>
    }
}
