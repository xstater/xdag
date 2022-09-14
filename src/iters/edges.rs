use std::collections::BTreeMap;

/// iterator of the edges in `DAG`
pub struct EdgesIter<'a, NodeId, EdgeData> {
    pub(crate) from_iter: std::collections::btree_map::Iter<'a, NodeId, BTreeMap<NodeId, EdgeData>>,
    pub(crate) to_iter: Option<(
        NodeId,
        std::collections::btree_map::Iter<'a, NodeId, EdgeData>,
    )>,
}

impl<'a, NodeId, EdgeData> Iterator for EdgesIter<'a, NodeId, EdgeData>
where
    NodeId: Copy,
{
    type Item = (NodeId, NodeId, &'a EdgeData);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((from_id, iter)) = self.to_iter.as_mut().as_mut() {
            let from_id = from_id.clone();
            if let Some((to_id, data)) = iter.next() {
                return Some((from_id, to_id.clone(), data));
            }
        }
        // yield None or to_iter is none
        if let Some((from_id, map)) = self.from_iter.next() {
            let to_iter = map.iter();
            self.to_iter.replace((from_id.clone(), to_iter));
            self.next()
        } else {
            None
        }
    }
}

pub struct EdgesIterMut<'a, NodeId, EdgeData> {
    pub(crate) from_iter:
        std::collections::btree_map::IterMut<'a, NodeId, BTreeMap<NodeId, EdgeData>>,
    pub(crate) to_iter: Option<(
        NodeId,
        std::collections::btree_map::IterMut<'a, NodeId, EdgeData>,
    )>,
}

impl<'a, NodeId, EdgeData> Iterator for EdgesIterMut<'a, NodeId, EdgeData>
where
    NodeId: Copy,
{
    type Item = (NodeId, NodeId, &'a mut EdgeData);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((from_id, iter)) = self.to_iter.as_mut().as_mut() {
            let from_id = from_id.clone();
            if let Some((to_id, data)) = iter.next() {
                return Some((from_id, to_id.clone(), data));
            }
        }
        // yield None or to_iter is none
        if let Some((from_id, map)) = self.from_iter.next() {
            let to_iter = map.iter_mut();
            self.to_iter.replace((from_id.clone(), to_iter));
            self.next()
        } else {
            None
        }
    }
}
