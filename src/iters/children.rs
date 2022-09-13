/// iterator of the children in `DAG`
pub struct ChildrenIter<'a, NodeId, EdgeData> {
    pub(crate) iter: Option<std::collections::hash_map::Iter<'a, NodeId, EdgeData>>,
}

impl<'a, NodeId, EdgeData> Iterator for ChildrenIter<'a, NodeId, EdgeData>
where
    NodeId: Copy,
{
    type Item = (NodeId, &'a EdgeData);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = &mut self.iter {
            iter.next().map(|(id, data)| (*id, data))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if let Some(iter) = &self.iter {
            iter.size_hint()
        } else {
            (0, Some(0))
        }
    }
}

impl<'a, NodeId, EdgeData> ExactSizeIterator for ChildrenIter<'a, NodeId, EdgeData> where
    NodeId: Copy
{
}

/// iterator of the children in `DAG`
pub struct ChildrenIterMut<'a, NodeId, EdgeData> {
    pub(crate) iter: Option<std::collections::hash_map::IterMut<'a, NodeId, EdgeData>>,
}

impl<'a, NodeId, EdgeData> Iterator for ChildrenIterMut<'a, NodeId, EdgeData>
where
    NodeId: Copy,
{
    type Item = (NodeId, &'a mut EdgeData);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = &mut self.iter {
            iter.next().map(|(id, data)| (*id, data))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if let Some(iter) = &self.iter {
            iter.size_hint()
        } else {
            (0, Some(0))
        }
    }
}

impl<'a, NodeId, EdgeData> ExactSizeIterator for ChildrenIterMut<'a, NodeId, EdgeData> where
    NodeId: Copy
{
}
