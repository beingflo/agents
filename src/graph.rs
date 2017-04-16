use std::collections::HashSet;
use std::cmp::max;

#[derive(Debug)]
pub struct Graph<T, S> {
    nodes: Vec<Node<T>>,
    edges: Vec<Edge<S>>,

    nodes_free: HashSet<NodeIndex>,
    edges_free: HashSet<EdgeIndex>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NodeIndex(usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct EdgeIndex(usize);

#[derive(Debug)]
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

#[derive(Debug)]
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

    pub fn num_nodes(&self) -> usize {
        max(self.nodes.len() - self.nodes_free.len(), 0)
    }

    // num_edges should not be relied on!
    // Incoming edges of deleted nodes are not explicity
    // removed, thus this function may return a number greater
    // than the actual active edges.
    // An accurate number may only be returned right after 
    // a call to 'remove_free'
    pub fn num_edges(&self) -> usize {
        max(self.edges.len() - self.edges_free.len(), 0)
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

    // Incoming edges are not deleted, they must be invalidated
    // eventually at a later time
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
        if self.nodes[source.0].free || self.nodes[source.0].free {
            return;
        }

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
            node.first = Some(edge_idx);
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
        let source_node = &self.nodes[source.0];
        let target_node = &self.nodes[target.0];

        if source_node.free || target_node.free {
            return false;
        }

        let mut edge = source_node.first;

        while let Some(edge_idx) = edge {
            let e = &self.edges[edge_idx.0];
            if e.target == target && !e.free {
                return true;
            }

            edge = e.next;
        }

        false
    }

    fn remove_free(&mut self) {
        unimplemented!();
    }

    fn node_payload(&mut self, node: NodeIndex) -> &mut T {
        &mut self.nodes[node.0].payload
    }

    fn edge_payload(&mut self, source: NodeIndex, target: NodeIndex) -> Option<&mut S> {
        let source_node = &self.nodes[source.0];
        let target_node = &self.nodes[target.0];

        if source_node.free || target_node.free {
            return None;
        }

        let mut edge = source_node.first;

        while let Some(edge_idx) = edge {
            if self.edges[edge_idx.0].target == target && !self.edges[edge_idx.0].free {
                return Some(&mut self.edges[edge_idx.0].payload);
            }
            edge = self.edges[edge_idx.0].next;
        }

        None
    }
}


#[cfg(test)]
mod tests {
    use graph::Graph;

    #[test]
    fn construction() {
        let mut g = Graph::<(),()>::new();

        for _ in 0..10 {
            let a = g.add_node(());
            let b = g.add_node(());

            g.add_edge(a, b, ());
        }

        assert_eq!(g.num_nodes(), 20);
        assert_eq!(g.num_edges(), 10);
    }

    #[test]
    fn remove_node() {
        let mut g = Graph::<(),()>::new();

        let mut a = g.add_node(());
        for _ in 0..10 {
            a = g.add_node(());
            let b = g.add_node(());

            g.add_edge(a, b, ());
        }

        g.remove_node(a);

        assert_eq!(g.num_nodes(), 20);
        assert_eq!(g.num_edges(), 9);
    }

    #[test]
    fn remove_edge() {
        let mut g = Graph::<(),()>::new();

        let mut a = g.add_node(());
        let mut b = g.add_node(());
        for _ in 0..10 {
            a = g.add_node(());
            b = g.add_node(());

            g.add_edge(a, b, ());
        }

        g.remove_edge(a, b);

        assert_eq!(g.num_nodes(), 22);
        assert_eq!(g.num_edges(), 9);
    }

    #[test]
    fn remove_both() {
        let mut g = Graph::<(),()>::new();

        let mut a = g.add_node(());
        let mut b = g.add_node(());
        for _ in 0..10 {
            a = g.add_node(());
            b = g.add_node(());

            g.add_edge(a, b, ());
        }

        assert_eq!(g.num_edges(), 10);

        g.remove_edge(a, b);

        assert_eq!(g.num_edges(), 9);

        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());

        g.add_edge(a, b, ());
        g.add_edge(a, c, ());
        g.add_edge(b, c, ());

        assert_eq!(g.num_edges(), 12);

        g.remove_node(a);

        assert_eq!(g.num_nodes(), 24);
        assert_eq!(g.num_edges(), 10);
    }

    #[test]
    fn contains() {
        let mut g = Graph::<(), ()>::new();

        let mut vec = Vec::new();
        for _ in 0..10 {
            vec.push(g.add_node(()));
        }

        for i in 0..10 {
            for k in 0..10 {
                g.add_edge(vec[i], vec[k], ());
           }
        }

        assert_eq!(g.num_edges(), 100);
        assert_eq!(g.contains_edge(vec[5], vec[8]), true);
        assert_eq!(g.contains_edge(vec[8], vec[5]), true);

        g.remove_node(vec[5]);

        assert_eq!(g.num_edges(), 90);
        assert_eq!(g.contains_edge(vec[5], vec[8]), false);
        // Incoming edges are not explicitly removed, but 
        // contains_edge reports correctly
        assert_eq!(g.contains_edge(vec[8], vec[5]), false);
    }
}
