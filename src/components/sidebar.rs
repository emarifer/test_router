use crate::router::Route;
use yew::prelude::{
    classes, function_component, html, Callback, Html, MouseEvent, Properties, UseStateHandle,
};
use yew_router::prelude::Link;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub activesidebar: bool,
    pub hidesidebar: Callback<MouseEvent>,
    pub activepath: UseStateHandle<String>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let overlay_section = if props.activesidebar {
        html!(<div onclick={props.hidesidebar.clone()} class="overlay"></div>)
    } else {
        html!()
    };

    html! {
      <>
        {overlay_section}
        <div class={classes!("sidebar", if props.activesidebar { "sidebar-visible" } else { "" })}>
            <button onclick={props.hidesidebar.clone()}><img src="img/x-lg.svg" alt="Close button" /></button>

            <div>
                <span class={classes!(if &*props.activepath == "/test_router/" {"active"} else {""})}>
                    <Link<Route> to={Route::Home}>
                        <span onclick={props.hidesidebar.clone()}>{"Home"}</span>
                    </Link<Route>>
                </span>

                <span class={classes!(if &*props.activepath == "/test_router/hello" {"active"} else {""})}>
                    <Link<Route> to={Route::Hello}>
                        <span onclick={props.hidesidebar.clone()}>{"Hello"}</span>
                    </Link<Route>>
                </span>
            </div>

            <div></div>
        </div>
      </>
    }
}

/*
 * https://stackoverflow.com/questions/40018606/auto-width-of-flex-column
 * https://developer.mozilla.org/en-US/docs/Web/CSS/:last-child
 */
