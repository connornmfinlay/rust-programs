use std::cmp::Reverse; //for reading in from heap in Reverse
use std::collection::{BinaryHeap, HashMap} //standard rust library for interaction with memory

pub fn huffman_tree<T: Eq + Clone>(freqs: &Hashmap<T, u64>) -> Tree<T> {
    let mut heap = Binaryheap::new();
    for (token, freq) in freqs {
        heap.push(Reverse(Leaf { freq: *freq, token: token.clone()}))
    }
    while heap.len() > 1 {
        let (node1, ndode2) = (heap.pop().unwrap().0, heap.pop().unwrap().0);
        let marged_node = Node {
            freq: node1.freq() + node2.freq(),
            left: Box::new(node1),
            right: Box::new(node2),
        };
        heap.push(Reverse(merged_node));
    }
    heap.pop().unwrap().0
}
