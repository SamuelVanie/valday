use yew::prelude::*;
use crate::app::Route;
use yew_router::prelude::*;

#[function_component(YesPage)]
pub fn yes_page() -> Html {

    html! {
        <div class="reasons">
            <img src="https://media4.giphy.com/media/v1.Y2lkPTc5MGI3NjExazJ4c2Qyc2tzbzFmMmh5anU4b3Z6M20zZTB5c2U0N24xeDVieDJzdSZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/UgNxy98zEpDiM/giphy.gif" alt="Cute GIF" />
        <p class="etre">{ "Yaaaaaay" }</p>
        <Link<Route> to={Route::Reasons}>
        <button class="fancy-button home">
                <span></span>
                <span><svg xmlns="http://www.w3.org/2000/svg" x="0px" y="0px" width="50" height="50" viewBox="0,0,256,256">
<g fill="#ffffff" fill-rule="nonzero" stroke="none" stroke-width="1" stroke-linecap="butt" stroke-linejoin="miter" stroke-miterlimit="10" stroke-dasharray="" stroke-dashoffset="0" font-family="none" font-weight="none" font-size="none" text-anchor="none" style="mix-blend-mode: normal"><g transform="scale(10.66667,10.66667)"><path d="M12,2.09961l-11,9.90039h3v9h7v-6h2v6h7v-9h3zM12,4.79102l6,5.40039v0.80859v8h-3v-6h-6v6h-3v-8.80859z"></path></g></g>
</svg></span>
        </button>
        </Link<Route>>
        </div>
    }
}
