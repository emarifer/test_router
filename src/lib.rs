mod components;
mod router;

use crate::components::{content::Content, sidebar::Sidebar};
use crate::router::Route;

use yew::{classes, function_component, html, use_state, Callback, Html};
use yew_router::prelude::{BrowserRouter, Link};

#[function_component(App)]
pub fn app() -> Html {
    let active_path = use_state(|| String::new());

    let show_sidebar = use_state(|| false);

    let hamburger_on_click = {
        let show_sidebar = show_sidebar.clone();

        Callback::from(move |_| {
            show_sidebar.set(!*show_sidebar);
        })
    };

    let hide_sidebar = {
        let show_sidebar = show_sidebar.clone();

        Callback::from(move |_| {
            show_sidebar.set(!*show_sidebar);
        })
    };

    html! {
            <BrowserRouter>
                <nav>
                    <span class={classes!(if &*active_path == "/test_router/" {"active"} else {""}, "custom-links")}>
                        <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                    </span>

                    <span class={classes!(if &*active_path == "/test_router/hello" {"active"} else {""}, "custom-links")}>
                        <Link<Route> to={Route::Hello}>{ "Hello" }</Link<Route>>
                    </span>

                    <button class="hamburger-button" onclick={hamburger_on_click}>
                        <img src="img/list.svg" alt="Hamburger button" />
                    </button>
                </nav>

                <Content activepath={active_path.clone()} />

                <Sidebar activesidebar={*show_sidebar} hidesidebar={hide_sidebar} activepath={active_path.clone()} />
            </BrowserRouter>
    }
}

// https://yew.rs/docs/concepts/router
// https://stackoverflow.com/questions/72731644/howto-add-is-active-css-class-when-click-route-link-in-yew
// https://dev.to/fazliddin04/react-router-v6-animated-transitions-diy-3e6l
// https://codesandbox.io/s/floral-wave-x6299o?from-embed

/*
 * https://stackoverflow.com/questions/40018606/auto-width-of-flex-column
 * https://developer.mozilla.org/en-US/docs/Web/CSS/:last-child
 */
