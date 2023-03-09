use crate::components::pages::{hello::Hello, home::Home, not_found::NotFound};

use yew::prelude::{html, Html};
use yew_router::prelude::Routable;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/test_router/")]
    Home,
    #[at("/test_router/hello")]
    Hello,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Hello => html! { <Hello /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
