if iswindows()
    loadlib("target/release/ring_hash.dll")
elseif ismacosx()
    loadlib("target/release/libring_hash.dylib")
else
    loadlib("target/release/libring_hash.so")
ok

? "=== Hash Demo - Using ring_module! Proc Macro ==="
? ""

? "--- Base64 Encoding/Decoding ---"
original = "Hello, Ring!"
encoded = hash_base64_encode(original)
decoded = hash_base64_decode(encoded)
? "Original: " + original
? "Encoded:  " + encoded
? "Decoded:  " + decoded
? ""

? "--- Hex Encoding/Decoding ---"
hex_encoded = hash_hex_encode("Hello")
hex_decoded = hash_hex_decode(hex_encoded)
? "Hex encoded 'Hello': " + hex_encoded
? "Hex decoded:         " + hex_decoded
? ""

? "--- Hashing Functions ---"
text = "The quick brown fox"
? "Text: " + text
? "MD5:    " + hash_md5(text)
? "SHA256: " + hash_sha256(text)
? "SHA512: " + hash_sha512(text)
? ""

? "--- Hasher Struct ---"
h = hash_hasher_new("md5")
? "Algorithm: " + hash_hasher_get_algorithm(h)
? "Hash:      " + hash_hasher_hash(h, "test")

hash_hasher_set_algorithm(h, "sha256")
? "Changed to: " + hash_hasher_get_algorithm(h)
? "Hash:       " + hash_hasher_hash(h, "test")

hash_hasher_delete(h)
? ""

? "=== All tests completed! ==="
