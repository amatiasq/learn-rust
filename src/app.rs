use super::data::create_graph;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct NodeViewProps {
    name: String,
    index: usize,
}

#[function_component]
fn NodeView(props: &NodeViewProps) -> Html {
    html! {
        <g transform={format!("translate(200, {})", (props.index + 1) * 150)}>
            <circle r="50" fill="white" />
            <text y="5" font-size="14" text-anchor="middle">{props.name.clone()}</text>
        </g>
    }
}

#[function_component]
pub fn App() -> Html {
    let graph = create_graph();

    html! {
        <svg style="width: 100svw; height: 100svh">
            { graph.nodes
                .iter()
                .enumerate()
                .map(|(i, node)| html! {<NodeView name={node.name} index={i} /> })
                .collect::<Html>()
            }
        </svg>
    }
}
