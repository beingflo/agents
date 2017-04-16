use std::collections::HashSet;
use std::cmp::max;
use std;

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
    target: NodeIndex,
    next: Option<EdgeIndex>,

    payload: S,
}

pub struct NodeIterator<'a, T: 'a, S: 'a> {
    graph: &'a Graph<T, S>,
    cur_idx: usize,
}

impl<'a, T: 'a, S: 'a> Iterator for NodeIterator<'a, T, S> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        println!("next with {}", self.cur_idx);
        if self.cur_idx >= self.graph.nodes.len() {
            return None;
        }
        println!("ok");

        if self.graph.nodes[self.cur_idx].free {
            self.cur_idx += 1;
            self.next()
        } else {
            let ref_T = Some(&self.graph.nodes[self.cur_idx].payload);
            self.cur_idx += 1;
            ref_T
        }
    }
}

impl<S> Edge<S> {
    fn new(target: NodeIndex, next: Option<EdgeIndex>, payload: S) -> Edge<S> {
        Edge { target: target, next: next, payload: payload }
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

    pub fn num_edges(&self) -> usize {
        max(self.edges.len() - self.edges_free.len(), 0)
    }

    pub fn nodes_iter<'a>(&'a self) -> NodeIterator<'a, T, S> {
        NodeIterator { graph: self, cur_idx: 0 }
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
        self.nodes[node_idx.0].free = true;

        let mut edge_idx = self.nodes[node_idx.0].first;

        // Remove outgoing edges
        while let Some(e_idx) = edge_idx {
            let e = &mut self.edges[e_idx.0];

            self.edges_free.insert(e_idx);

            edge_idx = e.next;
        }

        // Remove incoming edges
        for n in self.nodes.iter_mut() {
            let mut repeat = true;
            let mut edge_idx = n.first;
            let mut prev_idx = edge_idx;

            if edge_idx.is_none() {
                continue;
            }

            while repeat {
                let mut found = false;
                while let Some(e_idx) = edge_idx {
                    let edge = &mut self.edges[e_idx.0];

                    if edge.target == node_idx {
                        found = true;
                        break;
                    }

                    prev_idx = edge_idx;
                    edge_idx = edge.next;
                }

                if !found {
                    repeat = false;
                    continue;
                }

                let e_next;
                {
                    let e_idx = edge_idx.unwrap();
                    let e = &mut self.edges[e_idx.0];

                    e_next = e.next;

                    // Bookkeeping
                    self.edges_free.insert(e_idx);
                }

                // Resolve first and next references
                if prev_idx == edge_idx {
                    n.first = e_next;
                } else {
                    // Unwrap won't panic since there was a successor
                    self.edges[prev_idx.unwrap().0].next = e_next;
                }

                if e_next.is_none() {
                    repeat = false;
                } else {
                    edge_idx = e_next;
                    prev_idx = edge_idx;
                }
            }
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
            if e.target == target {
                return true;
            }

            edge = e.next;
        }

        false
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
            if self.edges[edge_idx.0].target == target {
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

        assert_eq!(g.num_edges(), 81);
        assert_eq!(g.contains_edge(vec[5], vec[8]), false);

        // Incoming edges are not explicitly removed, but 
        // contains_edge reports correctly
        assert_eq!(g.contains_edge(vec[8], vec[5]), false);
    }

    #[test]
    fn re_add_node() {
        let mut g = Graph::<(), ()>::new();

        let a = g.add_node(());
        let b = g.add_node(());

        g.add_edge(a, b, ());

        g.remove_node(b);

        let c = g.add_node(());

        assert_eq!(g.contains_edge(a, c), false);
    }

    #[test]
    fn double_edge() {
        let mut g = Graph::<(), ()>::new();

        let a = g.add_node(());
        let b = g.add_node(());

        g.add_edge(a, b, ());
        g.add_edge(a, b, ());

        assert_eq!(g.num_edges(), 2);

        g.remove_edge(a, b);
        
        assert_eq!(g.num_edges(), 1);
        assert_eq!(g.contains_edge(a, b), true);

        g.add_edge(a, b, ());

        g.remove_node(b);

        assert_eq!(g.num_edges(), 0);
    }

    #[test]
    fn delete_nonexisting_node() {
        let mut g = Graph::<(), ()>::new();

        let a = g.add_node(());
        g.add_node(());

        g.remove_node(a);
        g.remove_node(a);

        assert_eq!(g.num_nodes(), 1);
        assert_eq!(g.num_edges(), 0);
    }

    #[test]
    fn delete_nonexisting_edge() {
        let mut g = Graph::<(), ()>::new();

        let a = g.add_node(());
        let b = g.add_node(());

        g.add_edge(a, b, ());

        g.remove_edge(b, a);

        assert_eq!(g.num_nodes(), 2);
        assert_eq!(g.num_edges(), 1);
    }

    #[test]
    fn large_total_graph() {
        let mut g = Graph::<(), ()>::new();

        let mut nodes = Vec::new();
        for _ in 0..1e3 as usize {
            nodes.push(g.add_node(()));
        }

        for i in 0..1e3 as usize {
            for k in 0..1e3 as usize {
                g.add_edge(nodes[i], nodes[k], ());
            }
        }

        assert_eq!(g.num_nodes(), 1e3 as usize);
        assert_eq!(g.num_edges(), 1e6 as usize);

        g.remove_node(nodes[257]);

        assert_eq!(g.num_nodes(), 1e3 as usize - 1);
        assert_eq!(g.num_edges(), 999*999);
    }

    #[test]
    fn actual_payload() {
        let mut g = Graph::<i32, f32>::new();

        let a = g.add_node(0);
        let b = g.add_node(1);

        g.add_edge(a, b, 0.123);

        assert_eq!(g.node_payload(a), &mut 0);
        assert_eq!(g.node_payload(b), &mut 1);
        assert_eq!(g.edge_payload(a, b), Some(&mut 0.123));
        assert_eq!(g.edge_payload(b, a), None);
    }

    #[test]
    fn nodes_iter() {
        let mut g = Graph::<i32, ()>::new();

        for i in 0..5 {
            g.add_node(i);
        }

        assert_eq!(g.nodes_iter().collect::<Vec<_>>(), vec![&0, &1, &2, &3, &4]);
    }
}
