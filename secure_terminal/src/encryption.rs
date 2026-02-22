/// Encryption module for handling data encoding/decoding operations
/// Uses XOR cipher with a fixed secret key for simple obfuscation

const SECRET_KEY: u8 = 0x53;

/// Decrypts XOR-encoded data to a UTF-8 string
///
/// # Arguments
/// * `data` - Byte slice containing XOR-encrypted data
///
/// # Returns
/// Decrypted string, or "??" if UTF-8 conversion fails
pub fn decode(data: &[u8]) -> String {
    let decoded: Vec<u8> = data.iter().map(|&b| b ^ SECRET_KEY).collect();
    String::from_utf8(decoded).unwrap_or_else(|_| "??".to_string())
}

/// Encrypts a string using XOR cipher
///
/// # Arguments
/// * `text` - String to encrypt
///
/// # Returns
/// Vector of encrypted bytes
pub fn encode(text: &str) -> Vec<u8> {
    text.as_bytes().iter().map(|&b| b ^ SECRET_KEY).collect()
}

/// Encodes binary data to Base64
///
/// # Arguments
/// * `input` - Raw bytes to encode
///
/// # Returns
/// Base64 encoded string
pub fn base64_encode(input: &[u8]) -> String {
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();

    for chunk in input.chunks(3) {
        let b1 = chunk[0];
        let b2 = if chunk.len() > 1 { chunk[1] } else { 0 };
        let b3 = if chunk.len() > 2 { chunk[2] } else { 0 };

        let n = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);

        result.push(TABLE[((n >> 18) & 63) as usize] as char);
        result.push(TABLE[((n >> 12) & 63) as usize] as char);

        if chunk.len() > 1 {
            result.push(TABLE[((n >> 6) & 63) as usize] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(TABLE[(n & 63) as usize] as char);
        } else {
            result.push('=');
        }
    }

    result
}
