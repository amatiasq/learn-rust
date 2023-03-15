use super::data::create_graph;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let graph = create_graph();
    let x = graph.nodes.iter().map(|node| {
        html! {
            <div>
                <h1>{node.name}</h1>
            </div>
        }
    });

    html! {
        <main>
            {html! { for x }}
        </main>
    }
}
