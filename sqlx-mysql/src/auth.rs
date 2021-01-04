pub(crate) mod native;
// mod caching_sha2;
// mod sha256;

// XOR(x, y)
// If len(y) < len(x), wrap around inside y
fn xor_eq(x: &mut [u8], y: &[u8]) {
    let y_len = y.len();

    for i in 0..x.len() {
        x[i] ^= y[i % y_len];
    }
}
