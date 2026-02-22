fn main() {
    let key: u64 = 144935935482u64; // Change this secret key (must match encrypt.rs)
    let key_bytes = key.to_le_bytes();
    let scrambled: Vec<u8> = vec![68, 11, 85, 34, 18, 32, 26, 85, 39, 18, 31, 24, 20, 25, 20, 23, 85, 107, 37, 30, 36, 11, 85, 99, 10, 85, 23, 27, 20, 31, 10, 33, 11, 18, 27, 85, 33, 26, 31, 11, 25, 26, 23, 20, 26]; // CHANGE THIS TO YOUR ENCRYPTED BYTES
    let decrypted: String = scrambled
        .iter()
        .enumerate()
        .map(|(i, b)| (b ^ key_bytes[i % key_bytes.len()]) as char)
        .collect();
    println!("{}", decrypted);
}
