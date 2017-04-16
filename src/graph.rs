use std::collections::HashSet;

pub struct Graph<T, S> {
    nodes: Vec<Node<T>>,
    edges: Vec<Edge<S>>,

    nodes_free: HashSet<NodeIndex>,
    edges_free: HashSet<EdgeIndex>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct NodeIndex(usize);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct EdgeIndex(usize);

pub struct Node<T> {
    free: bool,
    first: Option<EdgeIndex>,

    payload: T,
}

impl<T> Node<T> {
    fn new(payload: T) -> Node<T> {
        Node { free: false, first: None, payload: payload }
    }
}

pub struct Edge<S> {
    free: bool,
    target: NodeIndex,
    next: Option<EdgeIndex>,

    payload: S,
}

impl<S> Edge<S> {
    fn new(target: NodeIndex, next: Option<EdgeIndex>, payload: S) -> Edge<S> {
        Edge { free: false, target: target, next: next, payload: payload }
    }
}

impl<T, S> Graph<T, S> {
    pub fn new() -> Graph<T, S> {
        Graph { nodes: Vec::new(), edges: Vec::new(),
                nodes_free: HashSet::new(), edges_free: HashSet::new() }
    }

    pub fn add_node(&mut self, payload: T) -> NodeIndex {
        if self.nodes_free.is_empty() {
            let index = self.nodes.len();
            self.nodes.push(Node::new(payload));

            NodeIndex(index)
        } else {
            let node_idx = *self.nodes_free.iter().nth(0).unwrap();
            self.nodes_free.remove(&node_idx);

            self.nodes[node_idx.0] = Node::new(payload);

            node_idx
        }
    }

    pub fn remove_node(&mut self, node_idx: NodeIndex) {
        self.nodes_free.insert(node_idx);
        let node = &mut self.nodes[node_idx.0];
        node.free = true;

        let mut edge_idx = node.first;

        while let Some(e_idx) = edge_idx {
            let e = &mut self.edges[e_idx.0];

            e.free = true;
            self.edges_free.insert(e_idx);

            edge_idx = e.next;
        }
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex, payload: S) {
        if self.edges_free.is_empty() {
            let index = self.edges.len();
            let node = &mut self.nodes[source.0];

            self.edges.push(Edge::new(target, node.first, payload));

            node.first = Some(EdgeIndex(index));
        } else {
            let edge_idx = *self.edges_free.iter().nth(0).unwrap();
            self.edges_free.remove(&edge_idx);

            let node = &mut self.nodes[source.0];
            self.edges[edge_idx.0] = Edge::new(target, node.first, payload);
        }
    }

    pub fn remove_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let node = &mut self.nodes[source.0];
        let mut edge_idx = node.first;

        // Keep track of previous edge to resolve dangling index
        let mut prev_idx = edge_idx;
        let mut found = false;
        while let Some(e_idx) = edge_idx {
            let edge = &self.edges[e_idx.0];
            if edge.target == target {
                found = true;
                break;
            }

            prev_idx = edge_idx;
            edge_idx = edge.next;
        }

        if !found {
            return;
        }

        let e_next;
        {
            let e_idx = edge_idx.unwrap();
            let e = &mut self.edges[e_idx.0];

            e_next = e.next;

            // Bookkeeping
            self.edges_free.insert(e_idx);
            e.free = true;
        }

        // Resolve first and next references
        if prev_idx == edge_idx {
            node.first = e_next;
        } else {
            // Unwrap won't panic since there was a successor
            self.edges[prev_idx.unwrap().0].next = e_next;
        }
    }

    fn contains_edge(&self, source: NodeIndex, target: NodeIndex) -> bool {
        let node_source = &self.nodes[source.0];
        let node_target = &self.nodes[target.0];

        if node_source.free || node_target.free {
            return true;
        }

        let mut edge = node_source.first;

        while let Some(edge_idx) = edge {
            let e = &self.edges[edge_idx.0];
            if e.target == target && !e.free {
                return true;
            }

            edge = e.next;
        }

        false
    }
}


#[cfg(test)]
mod tests {

}
