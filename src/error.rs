use std::{error::Error, fmt::Display};

/// Errors about `DAG`
#[derive(Debug, Clone, Copy)]
pub enum DagError<NodeId, EdgeData> {
    /// There is no such id in `DAG`
    NodeNotFound(NodeId),
    /// Insert this edge will make a cycle which whill destroy the `DAG`
    HasCycle(NodeId, NodeId, EdgeData),
}

impl<NodeId, EdgeData> Display for DagError<NodeId, EdgeData>
where
    NodeId: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DagError::NodeNotFound(id) => {
                writeln!(f, "Cannot found node in Dag where node_id='{}'.", id)
            }
            DagError::HasCycle(from, to, _) => writeln!(
                f,
                "DAG was destoryed since detected a cycle when insert edge '{}' -> '{}'",
                from, to
            ),
        }
    }
}

impl<NodeId, EdgeData> Error for DagError<NodeId, EdgeData>
where
    NodeId: std::fmt::Debug + Display,
    EdgeData: std::fmt::Debug,
{
}
