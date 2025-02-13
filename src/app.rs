use yew::prelude::*;
use yew_router::prelude::*;

use crate::reasons_page::ReasonsPage;
use crate::ask_page::AskPage;
use crate::yes_page::YesPage;

#[derive(Clone, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/")]
    Reasons,
    #[at("/ask")]
    Ask,
    #[at("/yes")]
    Yes,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Reasons => html! { <ReasonsPage /> },
        Route::Ask => html! { <AskPage /> },
        Route::Yes => html! { <YesPage /> },
        Route::NotFound => html! { <h1>{" Evite de trop bouger "}</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
