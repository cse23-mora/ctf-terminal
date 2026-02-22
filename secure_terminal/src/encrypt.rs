fn main() {
    let key: u64 = 144935935482u64; // Change this secret key (large number)
    let text = "Sangeeth_guest"; // CHANGE THIS TO WHATEVER YOU WANT
    let key_bytes = key.to_le_bytes();
    let scrambled: Vec<u8> = text
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(i, b)| b ^ key_bytes[i % key_bytes.len()])
        .collect();
    println!("vec!{:?}", scrambled);
}