# cisco7-rs
Cisco type 7 password de/encryptor written in Rust

Optionally compiles to WebAssembly

## Usage
```rs
use cisco7_rs::*;

// Encrypting
let plaintext = "1234";
let encrypted = encrypt(plaintext, None).unwrap(); // Can replace None with Some(<desired salt>)

// Decrypting
let password = "08701E1D5D";
let decrypted = decrypt(password).unwrap();
```

## Compiling to WebAssembly
The best way I know to compile this to WASM is: 
1. Downloading `wasm-pack`
2. `wasm-pack build -t web --features wasm`, optionally specifying `-d <output directory>`
