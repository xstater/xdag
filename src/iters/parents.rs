/// iterator of the parents in `DAG`
pub struct ParentsIter<'a, NodeId> {
    pub(crate) iter: Option<std::collections::btree_set::Iter<'a, NodeId>>,
}

impl<'a, NodeId> Iterator for ParentsIter<'a, NodeId>
where
    NodeId: Copy,
{
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = &mut self.iter {
            iter.next().copied()
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

impl<'a, NodeId> ExactSizeIterator for ParentsIter<'a, NodeId> where NodeId: Copy {}
