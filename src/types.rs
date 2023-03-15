pub enum NodeType {
    Person,
    Place,
    // Thing,
    // Moment,
    // Event,
}

pub enum EdgeType {
    Marriage,
    ChildOf,
    BeenThere,
    // Friend,
}

pub struct Node<'a> {
    pub id: u64,
    pub node_type: NodeType,
    pub name: &'a str,
}

pub struct Edge {
    pub id: u64,
    pub from: u64,
    pub to: u64,
    pub edge_type: EdgeType,
}

pub struct Graph<'a> {
    pub nodes: Vec<Node<'a>>,
    pub edges: Vec<Edge>,
}
