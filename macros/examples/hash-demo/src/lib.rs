use base64::{engine::general_purpose::STANDARD, Engine};
use md5::{Digest as Md5Digest, Md5};
use ring_lang_codegen::ring_extension;
use ring_lang_rs::*;
use sha2::{Sha256, Sha512};

ring_extension! {
    prefix: "hash";

    pub fn base64_encode(input: &str) -> String {
        STANDARD.encode(input.as_bytes())
    }

    pub fn base64_decode(input: &str) -> String {
        STANDARD.decode(input).map(|b| String::from_utf8_lossy(&b).to_string()).unwrap_or_default()
    }

    pub fn md5(input: &str) -> String {
        let mut hasher = Md5::new();
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn sha256(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn sha512(input: &str) -> String {
        let mut hasher = Sha512::new();
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn hex_encode(input: &str) -> String {
        hex::encode(input.as_bytes())
    }

    pub fn hex_decode(input: &str) -> String {
        hex::decode(input).map(|b| String::from_utf8_lossy(&b).to_string()).unwrap_or_default()
    }

    #[derive(Default)]
    pub struct Hasher {
        pub algorithm: String,
    }

    impl Hasher {
        pub fn new(algorithm: &str) -> Self {
            Hasher { algorithm: algorithm.to_string() }
        }

        pub fn hash(&self, input: &str) -> String {
            match self.algorithm.as_str() {
                "md5" => { let mut h = Md5::new(); h.update(input.as_bytes()); hex::encode(h.finalize()) }
                "sha256" => { let mut h = Sha256::new(); h.update(input.as_bytes()); hex::encode(h.finalize()) }
                "sha512" => { let mut h = Sha512::new(); h.update(input.as_bytes()); hex::encode(h.finalize()) }
                _ => String::from("unknown algorithm")
            }
        }

        pub fn set_algorithm(&mut self, algorithm: &str) {
            self.algorithm = algorithm.to_string();
        }
    }
}
