fn encoder_to_dencoder<T: Clone> (
    encoder: &HashMap<T, BitVec>
) -> HashMap<BitVec, T> {
    let mut decoder = HashMap::new();

    for (token, prefix) in encoder.clone() {
        decoder.insert(prefix, token);
    }
    decoder
}
