use super::types::*;

const ME: Node = Node {
    id: 1,
    node_type: NodeType::Person,
    name: "A. Matías Quezada",
};

const WIFE: Node = Node {
    id: 2,
    node_type: NodeType::Person,
    name: "Wife",
};

const SON: Node = Node {
    id: 3,
    node_type: NodeType::Person,
    name: "Son",
};

const DAUGTHER: Node = Node {
    id: 4,
    node_type: NodeType::Person,
    name: "Daughter",
};

const HOME: Node = Node {
    id: 5,
    node_type: NodeType::Place,
    name: "Home",
};

const WORK: Node = Node {
    id: 6,
    node_type: NodeType::Place,
    name: "Work",
};

const ME_WIFE: Edge = Edge {
    id: 1,
    from: ME.id,
    to: WIFE.id,
    edge_type: EdgeType::Marriage,
};

const ME_SON: Edge = Edge {
    id: 2,
    from: SON.id,
    to: ME.id,
    edge_type: EdgeType::ChildOf,
};

const ME_DAUGTHER: Edge = Edge {
    id: 3,
    from: DAUGTHER.id,
    to: ME.id,
    edge_type: EdgeType::ChildOf,
};

const WIFE_SON: Edge = Edge {
    id: 4,
    from: SON.id,
    to: WIFE.id,
    edge_type: EdgeType::ChildOf,
};

const WIFE_DAUGTHER: Edge = Edge {
    id: 4,
    from: DAUGTHER.id,
    to: WIFE.id,
    edge_type: EdgeType::ChildOf,
};

const ME_HOME: Edge = Edge {
    id: 5,
    from: ME.id,
    to: HOME.id,
    edge_type: EdgeType::BeenThere,
};

const ME_WORK: Edge = Edge {
    id: 6,
    from: ME.id,
    to: WORK.id,
    edge_type: EdgeType::BeenThere,
};

pub fn create_graph<'a>() -> Graph<'a> {
    Graph {
        nodes: vec![ME, WIFE, SON, DAUGTHER, HOME, WORK],
        edges: vec![
            ME_WIFE,
            ME_SON,
            ME_DAUGTHER,
            WIFE_SON,
            WIFE_DAUGTHER,
            ME_HOME,
            ME_WORK,
        ],
    }
}
