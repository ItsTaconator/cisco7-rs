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

## Special thanks to
- My IT instructor for mentioning that Cisco's `password-encryption` service is weak when teaching us about IOS
- [Eduardo Ortega Naranjo](https://github.com/Eduardo-Ortega102) for his [simple decryptor](https://gist.github.com/Eduardo-Ortega102/12923a72097347650856401eced0eb4d)
- [GIAC](https://www.giac.org/) for disclosing the vulnerability in [great detail](https://www.giac.org/paper/gcih/84/cisco-ios-type-7-password-vulnerability/100566) so I could figure out encrypting
