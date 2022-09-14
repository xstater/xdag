//! # XDag
//! A simple DAG (Directed Acyclic Graph) libarary
//! # Note
//! This lib just provides a data-structure to store DAG with checking.
//! It doesn't contain any algorithm about DAG
//! # Details
//! XDAG stores DAG by BTreeMap. Because it can ensure the order of edges and nodes.
//! # Some Examples
//! ```Rust
//! // Create a new DAG
//! let mut dag = Dag::new();
//! // insert 3 nodes with data '()'
//! dag.insert_node(2, ());
//! dag.insert_node(4, ());
//! dag.insert_node(3, ());
//! // insert 2 edges with data '()'
//! dag.insert_edge(2, 3, ()).unwrap();
//! dag.insert_edge(2, 4, ()).unwrap();
//! // Get all roots and leaves in DAG
//! let roots = dag.roots().map(|(id, _)| id).collect::<Vec<_>>();
//! let leaves = dag.leaves().map(|(id, _)| id).collect::<Vec<_>>();

//! assert_eq!(&roots, &[2]);
//! for id in [3, 4].iter() {
//!     assert!(leaves.contains(id))
//! }
//! ```
mod error;
pub mod iters;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};

pub use error::DagError;
use iters::{ChildrenIter, ChildrenIterMut, EdgesIter, EdgesIterMut, ParentsIter};

/// DAG
/// # Remarks
/// * You can store data in 'Node' or 'Edge'
/// * `NodeId` must be `Copy + Hash + Eq` because DAG is stored by `HashMap` and `HashSet`
#[derive(Debug, Clone)]
pub struct Dag<NodeId, NodeData, EdgeData> {
    nodes: BTreeMap<NodeId, NodeData>,
    edges: BTreeMap<NodeId, BTreeMap<NodeId, EdgeData>>,
    back_edges: BTreeMap<NodeId, BTreeSet<NodeId>>,
}

impl<NodeId, NodeData, EdgeData> Dag<NodeId, NodeData, EdgeData>
where
    NodeId: Copy + Ord
{
    /// Create an empty DAG
    pub fn new() -> Self {
        Dag {
            nodes: BTreeMap::new(),
            edges: BTreeMap::new(),
            back_edges: BTreeMap::new(),
        }
    }

    /// Check if a node is in a cycle, this will destory DAG
    fn in_cycle(&self, node_id: NodeId) -> bool {
        // DFS
        let mut visited = BTreeSet::new();
        let mut stack = vec![node_id];

        while let Some(top) = stack.pop() {
            if visited.contains(&top) {
                return true;
            }
            visited.insert(top);
            for child_id in self.children(top).map(|(id, _)| id) {
                stack.push(child_id)
            }
        }

        false
    }

    /// Check if a `node_id` is contained in `Dag`
    pub fn contains_node(&self, node_id: NodeId) -> bool {
        self.nodes.contains_key(&node_id)
    }

    /// Insert a node with data
    /// # Returns
    /// * Return `Some(data)` when `node_id` is already in `Dag`
    pub fn insert_node(&mut self, node_id: NodeId, node_data: NodeData) -> Option<NodeData> {
        if !self.edges.contains_key(&node_id) {
            self.edges.insert(node_id, BTreeMap::new());
        }
        if !self.back_edges.contains_key(&node_id) {
            self.back_edges.insert(node_id, BTreeSet::new());
        }
        self.nodes.insert(node_id, node_data)
    }

    /// Check if an `edge` is contained in `Dag`
    pub fn contains_edge(&self, from: NodeId, to: NodeId) -> bool {
        if let Some(children) = self.edges.get(&from) {
            return children.contains_key(&to);
        }
        false
    }

    /// Insert an edge with data in `Dag`
    /// # Return
    /// * Return `Ok(Some(data))` when there is a same edge in `Dag`
    /// # Errors
    /// * `Err(NodeNotFound(id))` when `from` or `to` is NOT found in `Dag`
    /// * `Err(HasCycle(from,to,data))` when a cycle is detected
    pub fn insert_edge(
        &mut self,
        from: NodeId,
        to: NodeId,
        edge_data: EdgeData,
    ) -> Result<Option<EdgeData>, DagError<NodeId, EdgeData>> {
        if !self.nodes.contains_key(&from) {
            return Err(DagError::NodeNotFound(from));
        }
        if !self.nodes.contains_key(&to) {
            return Err(DagError::NodeNotFound(to));
        }
        let children = self
            .edges
            .get_mut(&from)
            .unwrap_or_else(|| unreachable!("proved by contains_key"));
        let result = children.insert(to, edge_data);
        if self.in_cycle(from) {
            // roll back
            // remove that edge
            let children = self
                .edges
                .get_mut(&from)
                .unwrap_or_else(|| unreachable!("proved by contains_key"));
            let data = children
                .remove(&to)
                .unwrap_or_else(|| unreachable!("proved by contains_key"));
            return Err(DagError::HasCycle(from, to, data));
        }
        // added back edge
        let parents = self
            .back_edges
            .get_mut(&to)
            .unwrap_or_else(|| unreachable!("proved by contains_key"));
        parents.insert(from);
        Ok(result)
    }

    /// Remove an edge from `Dag`
    /// # Returns
    /// * Return `Ok(Some(data))` when success
    /// * Return `Ok(None)` when there is no such edge
    /// # Errors
    /// * `Err(NodeNotFound(id))` when `from` or `to` is NOT found in `Dag`
    pub fn remove_edge(
        &mut self,
        from: NodeId,
        to: NodeId,
    ) -> Result<Option<EdgeData>, DagError<NodeId, EdgeData>> {
        if !self.nodes.contains_key(&from) {
            return Err(DagError::NodeNotFound(from));
        }
        if !self.nodes.contains_key(&to) {
            return Err(DagError::NodeNotFound(to));
        }
        let children = self
            .edges
            .get_mut(&from)
            .unwrap_or_else(|| unreachable!("proved by contains_key"));
        let result = children.remove(&to);
        let parents = self
            .back_edges
            .get_mut(&to)
            .unwrap_or_else(|| unreachable!("proved by contains_key"));
        parents.remove(&from);
        Ok(result)
    }

    /// remove a node and all edges related
    /// # Returns
    /// * Return `(Some(data),edges_data)` if successed
    pub fn remove_node(&mut self, node_id: NodeId) -> (Option<NodeData>, Vec<EdgeData>) {
        if !self.contains_node(node_id) {
            return (None, Vec::new());
        }
        let mut edge_datas = Vec::new();
        // remove children edges
        let ids = self.children(node_id).map(|(id, _)| id).collect::<Vec<_>>();
        for child_id in ids {
            let data = self
                .remove_edge(node_id, child_id)
                .unwrap_or_else(|_| {
                    unreachable!(
                        "Xdag ensures this node exists both in nodes and edges at the same time"
                    )
                })
                .unwrap_or_else(|| {
                    unreachable!("data is from self.children, so there must be such an edge")
                });
            edge_datas.push(data);
        }
        // remove parents edges
        let ids = self.parents(node_id).collect::<Vec<_>>();
        for parent_id in ids {
            let data = self
                .remove_edge(parent_id, node_id)
                .unwrap_or_else(|_| {
                    unreachable!(
                        "Xdag ensures this node exists both in nodes and edges at the same time"
                    )
                })
                .unwrap_or_else(|| {
                    unreachable!("data is from self.parents, so there must be such an edge")
                });
            edge_datas.push(data)
        }
        // remove node
        let node_data = self.nodes.remove(&node_id);
        (node_data, edge_datas)
    }

    /// Get an iterator of all the children of given `node_id`
    pub fn children(&self, node_id: NodeId) -> ChildrenIter<'_, NodeId, EdgeData> {
        ChildrenIter {
            iter: self.edges.get(&node_id).map(|map| map.iter()),
        }
    }

    /// Get an iterator of all the children of given `node_id`
    pub fn children_mut(&mut self, node_id: NodeId) -> ChildrenIterMut<'_, NodeId, EdgeData> {
        ChildrenIterMut {
            iter: self.edges.get_mut(&node_id).map(|map| map.iter_mut()),
        }
    }

    /// Get an iterator of all the children of given `node_id`
    pub fn parents(&self, node_id: NodeId) -> ParentsIter<'_, NodeId> {
        ParentsIter {
            iter: self.back_edges.get(&node_id).map(|set| set.iter()),
        }
    }

    /// Get the count of nodes
    pub fn nodes_len(&self) -> usize {
        self.nodes.len()
    }

    /// Get all the nodes in `Dag`
    pub fn nodes(&self) -> impl Iterator<Item = (NodeId, &'_ NodeData)> {
        self.nodes.iter().map(|(id, data)| (*id, data))
    }

    /// Get all the nodes in `Dag`
    pub fn nodes_mut(&mut self) -> impl Iterator<Item = (NodeId, &'_ mut NodeData)> {
        self.nodes.iter_mut().map(|(id, data)| (*id, data))
    }

    /// Get all the edges in `Dag`
    pub fn edges(&self) -> EdgesIter<'_, NodeId, EdgeData> {
        EdgesIter {
            from_iter: self.edges.iter(),
            to_iter: None,
        }
    }

    /// Get all the edges in `Dag`
    pub fn edges_mut(&mut self) -> EdgesIterMut<'_, NodeId, EdgeData> {
        EdgesIterMut {
            from_iter: self.edges.iter_mut(),
            to_iter: None,
        }
    }

    /// Get all the leaves in `Dag`
    pub fn leaves(&self) -> impl Iterator<Item = (NodeId, &'_ NodeData)> {
        self.nodes().filter(|(id, _)| self.children(*id).len() == 0)
    }

    /// Get all the roots in `Dag`
    pub fn roots(&self) -> impl Iterator<Item = (NodeId, &'_ NodeData)> {
        self.nodes().filter(|(id, _)| self.parents(*id).len() == 0)
    }

    /// Get data from node
    /// # Returns
    /// Return `None` if `node_id` is not found in `Dag`
    pub fn get_node(&self, node_id: NodeId) -> Option<&NodeData> {
        self.nodes.get(&node_id)
    }

    /// Get mutable data from node
    /// # Returns
    /// Return `None` if `node_id` is not found in `Dag`
    pub fn get_node_mut(&mut self, node_id: NodeId) -> Option<&mut NodeData> {
        self.nodes.get_mut(&node_id)
    }

    /// Get data from edge
    /// # Returns
    /// Return `Ok(Some(data))` if success
    /// # Errors
    /// * `Err(NodeNotFound(id))` when `from` or `to` is NOT found in `Dag`
    pub fn get_edge(
        &self,
        from: NodeId,
        to: NodeId,
    ) -> Result<Option<&EdgeData>, DagError<NodeId, EdgeData>> {
        if !self.nodes.contains_key(&from) {
            return Err(DagError::NodeNotFound(from));
        }
        if !self.nodes.contains_key(&to) {
            return Err(DagError::NodeNotFound(to));
        }
        let children = self
            .edges
            .get(&from)
            .unwrap_or_else(|| unreachable!("proved by contains_key"));
        Ok(children.get(&to))
    }

    /// Get mutable data from edge
    /// # Returns
    /// Return `Ok(Some(data))` if success
    /// # Errors
    /// * `Err(NodeNotFound(id))` when `from` or `to` is NOT stored in `Dag`
    pub fn get_edge_mut(
        &mut self,
        from: NodeId,
        to: NodeId,
    ) -> Result<Option<&mut EdgeData>, DagError<NodeId, EdgeData>> {
        if !self.nodes.contains_key(&from) {
            return Err(DagError::NodeNotFound(from));
        }
        if !self.nodes.contains_key(&to) {
            return Err(DagError::NodeNotFound(to));
        }
        let children = self
            .edges
            .get_mut(&from)
            .unwrap_or_else(|| unreachable!("proved by contains_key"));
        Ok(children.get_mut(&to))
    }
}
