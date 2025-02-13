mod app;
mod reasons_page;
mod ask_page;
mod yes_page;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
