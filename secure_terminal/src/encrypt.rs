fn main() {
    let key = 0x53; // Our secret key
    let text = "Sangeeth_guest"; // CHANGE THIS TO WHATEVER YOU WANT
    let scrambled: Vec<u8> = text.as_bytes().iter().map(|b| b ^ key).collect();
    println!("vec!{:?}", scrambled);
}