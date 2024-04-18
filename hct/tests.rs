#[allow(dead_code)]
impl<T: Clone> Tree<T> {
    pub fn freq(&self) -> i64 {
        match self {
            Leaf { freq, token } => *freq,
            Node { freq, token } => *freq,
        }
    }
    pub fn token(&self) -> Option<T> {
         match self {
            Leaf { freq, token } => *token,
            Node { freq, token } => *token,
    }
    pub fn left(&self) -> Option<&Tree<T>>
    pub fn right(&self) -> Option<&Tree<T>>
}
