use yew::prelude::*;
use crate::app::Route;
use yew_router::prelude::*;

#[function_component(AskPage)]
pub fn ask_page() -> Html {

    let non_visible = use_state(|| false); // Initially visible

    let on_non_click = {
        let non_visible = non_visible.clone();
        Callback::from(move |_| {
            non_visible.set(true); // Set state to invisible on click
        })
    };

    let non_class = if *non_visible {
        "fancy-button accept faded" // Add 'faded' class
    } else {
        "fancy-button accept"
    };
    
    html! {
        <div class="reasons">
            <img class="gif" src="https://media4.giphy.com/media/v1.Y2lkPTc5MGI3NjExbnM1dG15eDdzdGI3eTJhYmZvOWZnYjgxdWVjZ2UzOGlydmNlazMwZyZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/D46lamuVD9n0s/giphy.gif" alt="Cute GIF" />
            <p class="etre">{ "Tu veux etre ma valentine?" }</p>
        <div>
        <Link<Route> to={Route::Yes}>
                <button class="fancy-button accept">
                        <span></span>
                        <span>{ "OUI" }</span>
                </button>
        </Link<Route>>
                <button class={non_class} onclick={on_non_click}>
                        <span></span>
                        <span>{ "NON" }</span>
        </button>
            </div>
        </div>
    }
}
