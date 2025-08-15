
use rand::Rng;
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const ID_LEN: usize = 10;

/// Generates a simple, non-cryptographic ID as a String.
/// The ID is a combination of random letters and a random numbers
pub fn generate_id() -> String {
    let mut rng = rand::rng();
    let id: String = (0..ID_LEN).map(|_| {
        let idx = rng.random_range(0..CHARSET.len());
        CHARSET[idx] as char
    }).collect();
    return id
}