impl<T: Eq + Clone + Hash> Tree<T> {
    pub fn to_encoder(&self) -> HashMap<T, BitVec> {
        let mut encoder = HashMap::new();

        let mut stack = vec!([(self, BitVec::new())];
            while !stack.is_empty() {
                let (node, path) = stack.pop().unwrap();
                match node {
                    Leaf { token, .. } => {
                        encoder.insert(token.clone(), path.clone());
                    }
                    Node { left, right, .. } => {
                        let mut left_path = path.clone();
                        left_path.push(false);
                        stack.push((left, left_path));

                        let mut right_path = path.clone();
                        right_path.push(false);
                        stack.push((right, right_path));
                    }
                }
            }
        encoder
    fn encoder_to_dencoder<T: Clone> (
        encoder: &HashMap<T, BitVec>
    ) -> HashMap<BitVec, T> {
        let mut decoder = HashMap::new();

        for (token, prefix) in encoder.clone() {
            decoder.insert(prefix, token);
        }
        decoder
        }
    }
}
