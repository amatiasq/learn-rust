mod app;
mod data;
mod types;

use app::App;

#[allow(non_snake_case)]

fn main() {
    yew::Renderer::<App>::new().render();

    println!("Hello, world!")
}
