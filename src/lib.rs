#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

/// XOR table
const XLAT: [u8; 53] = [
    0x64, 0x73, 0x66, 0x64, 0x3B, 0x6B, 0x66, 0x6F, 0x41, 0x2C, 0x2E, 0x69, 0x79, 0x65, 0x77, 0x72,
    0x6B, 0x6C, 0x64, 0x4A, 0x4B, 0x44, 0x48, 0x53, 0x55, 0x42, 0x73, 0x67, 0x76, 0x63, 0x61, 0x36,
    0x39, 0x38, 0x33, 0x34, 0x6E, 0x63, 0x78, 0x76, 0x39, 0x38, 0x37, 0x33, 0x32, 0x35, 0x34, 0x6B,
    0x3B, 0x66, 0x67, 0x38, 0x37,
];

/// Helper function to convert from string to hex
fn hex(str: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(str, 16)
}

/// Decrypts a Cisco type 7 password
/// 
/// Returns `None` if salt is invalid
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn decrypt(password: &str) -> Option<String> {
    let mut decrypted = String::new();
    let salt = password[..2].parse::<usize>();
    if salt.is_err() {
        println!("Salt is not a number");   
        return None;
    }

    let mut salt = salt.unwrap();

    if salt > 52 {
        println!("Salt too high");
        return None;
    }

    let text = &password[2..];
    for i in (0..text.len()).step_by(2) {
        let text_char = hex(&text[i..i + 2]).unwrap();
        let xlat_char = &XLAT[salt];

        decrypted.push((text_char ^ xlat_char) as char);

        salt += 1 % XLAT.len();
    }

    Some(decrypted)
}

/// Encrypts plaintext into a Cisco type 7 password
/// 
/// Returns `None` if desired salt is invalid
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn encrypt(plaintext: &str, salt: Option<usize>) -> Option<String> {
    let mut salt = salt.unwrap_or(8);
    if salt + plaintext.len() > 52 {
        println!("Salt too high");
        return None;
    }

    let password = plaintext.trim();
    let mut encrypted = String::new();
    encrypted.push_str(&format!("{:0>2}", salt));

    let bytes = password.as_bytes();    

    for i in 0..password.len() {
        encrypted.push_str(&format!("{:0>2X}", bytes[i] ^ XLAT[salt]));
        salt += 1;
    }
    
    Some(encrypted)
}

#[cfg(test)]
mod tests {
    #[test]
    fn encrypt() {
        const PLAINTEXT: &str = "1234";
        assert!(&super::encrypt(PLAINTEXT, Some(49)).unwrap() == "08701E1D5D")
    }

    #[test]
    fn decrypt() {
        const ENCRYPTED: &str = "08701E1D5D";
        assert!(&super::decrypt(ENCRYPTED).unwrap() == "1234")
    }
}
