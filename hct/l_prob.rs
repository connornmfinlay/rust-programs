use std::cmp::Ordering;

impl<T: Clone + Eq> Ord for Tree<T> {
    fn cmp(&self, other: &Self) -> Ordering { //compare
        self.freq().cmp(&other.freq())
    }
}

impl<T: Clone + Eq> PartialOrd for Tree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { //partial compare
    Some(self.cmp(other))
    }
}
