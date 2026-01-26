# Test script for ring_hash using Ring classes

# Load the library
if iswindows()
    loadlib("target/release/ring_hash.dll")
elseif ismacosx()
    loadlib("target/release/libring_hash.dylib")
else
    loadlib("target/release/libring_hash.so")
ok

# Load generated Ring classes
load "hash_classes.ring"

? "=== Testing Ring Classes ==="
? ""

# Test Hasher class
? "--- Hasher Class ---"

# Create with md5 algorithm
h = new Hasher("md5")
? "Created Hasher with algorithm: " + h.algorithm()
? "hash('test'): " + h.hash("test")

# Change algorithm using setter
h.setAlgorithm("sha256")
? "Changed algorithm to: " + h.algorithm()
? "hash('test'): " + h.hash("test")

# Change to sha512
h.set_algorithm("sha512")
? "Changed algorithm to: " + h.get_algorithm()
? "hash('test'): " + h.hash("test")

# Clean up
h.delete()
? ""

? "=== All class tests completed! ==="
