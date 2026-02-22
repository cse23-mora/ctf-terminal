use std::fs;

fn main() {
    let key: u64 = 144935935482u64; // Change this secret key (large number)
    
    // Read text from myencrypt.txt file
    let text = match fs::read_to_string("myencrypt.txt") {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading myencrypt.txt: {}", err);
            return;
        }
    };
    
    let key_bytes = key.to_le_bytes();
    let scrambled: Vec<u8> = text
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(i, b)| b ^ key_bytes[i % key_bytes.len()])
        .collect();
    println!("vec!{:?}", scrambled);
}