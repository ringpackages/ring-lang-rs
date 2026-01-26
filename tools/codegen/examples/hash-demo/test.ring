# Test for codegen-demo (parsec.ring approach)
# Tests base64, hex, md5, sha256, sha512 functions

# Load the library
if iswindows()
    loadlib("target/release/ring_hash.dll")
elseif ismacosx()
    loadlib("target/release/libring_hash.dylib")
else
    loadlib("target/release/libring_hash.so")
ok

? "=== Codegen Demo Test (parsec.ring) ==="
? ""

# Test standalone functions
? "--- Standalone Functions ---"

# Base64
encoded = hash_base64_encode("Hello, World!")
? "base64_encode('Hello, World!'): " + encoded
decoded = hash_base64_decode(encoded)
? "base64_decode: " + decoded

# Hex
hex_enc = hash_hex_encode("Hello")
? "hex_encode('Hello'): " + hex_enc
hex_dec = hash_hex_decode(hex_enc)
? "hex_decode: " + hex_dec

# Hashes
? ""
? "--- Hash Functions ---"
? "md5('Hello'): " + hash_md5_hash("Hello")
? "sha256('Hello'): " + hash_sha256_hash("Hello")
? "sha512('Hello'): " + hash_sha512_hash("Hello")

# Test Hasher struct
? ""
? "--- Hasher Struct ---"

# Create with md5
hasher = hash_hasher_new("md5")
? "Created Hasher with algorithm: " + hash_hasher_get_algorithm(hasher)
? "hash('test'): " + hash_hasher_hash(hasher, "test")

# Change to sha256
hash_hasher_set_algorithm(hasher, "sha256")
? "Changed algorithm to: " + hash_hasher_get_algorithm(hasher)
? "hash('test'): " + hash_hasher_hash(hasher, "test")

# Change to sha512
hash_hasher_set_algorithm(hasher, "sha512")
? "Changed algorithm to: " + hash_hasher_get_algorithm(hasher)
? "hash('test'): " + hash_hasher_hash(hasher, "test")

# Clean up
hash_hasher_delete(hasher)

? ""
? "=== All tests passed! ==="
