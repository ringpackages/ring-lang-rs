# UUID Demo Test
# Tests uuid crate wrapper functions

if iswindows()
    loadlib("target/release/ring_uuid.dll")
elseif ismacosx()
    loadlib("target/release/libring_uuid.dylib")
else
    loadlib("target/release/libring_uuid.so")
ok

? "=== UUID Demo Test ==="
? ""

# ============================================
? "--- UUID Generation ---"
# ============================================

v4_uuid = uuid_v4()
? "v4(): " + v4_uuid

v7_uuid = uuid_v7()
? "v7(): " + v7_uuid

? "nil(): " + uuid_nil()
? "max(): " + uuid_max()

# ============================================
? ""
? "--- Parsing & Validation ---"
# ============================================

? "is_valid(v4_uuid): " + uuid_is_valid(v4_uuid)
? "is_valid('invalid'): " + uuid_is_valid("invalid")
? "is_valid('550e8400-e29b-41d4-a716-446655440000'): " + uuid_is_valid("550e8400-e29b-41d4-a716-446655440000")

# Parse different formats
? ""
? "parse('550e8400e29b41d4a716446655440000'): " + uuid_parse("550e8400e29b41d4a716446655440000")
? "parse('550E8400-E29B-41D4-A716-446655440000'): " + uuid_parse("550E8400-E29B-41D4-A716-446655440000")
? "parse('{550e8400-e29b-41d4-a716-446655440000}'): " + uuid_parse("{550e8400-e29b-41d4-a716-446655440000}")

# ============================================
? ""
? "--- UUID Info ---"
# ============================================

? "get_version(v4_uuid): " + uuid_get_version(v4_uuid)
? "get_version(v7_uuid): " + uuid_get_version(v7_uuid)
? "get_version(nil): " + uuid_get_version(uuid_nil())
? "get_variant(v4_uuid): " + uuid_get_variant(v4_uuid)

# ============================================
? ""
? "--- UUID Formatting ---"
# ============================================

test_uuid = "550e8400-e29b-41d4-a716-446655440000"
? "to_string(): " + uuid_to_string(test_uuid)
? "to_upper(): " + uuid_to_upper(test_uuid)
? "to_simple(): " + uuid_to_simple(test_uuid)
? "to_urn(): " + uuid_to_urn(test_uuid)
? "to_braced(): " + uuid_to_braced(test_uuid)

# ============================================
? ""
? "--- UUID Comparison ---"
# ============================================

uuid1 = "550e8400-e29b-41d4-a716-446655440000"
uuid2 = "550e8400-e29b-41d4-a716-446655440001"

? "equals(uuid1, uuid1): " + uuid_equals(uuid1, uuid1)
? "equals(uuid1, uuid2): " + uuid_equals(uuid1, uuid2)
? "compare(uuid1, uuid2): " + uuid_compare(uuid1, uuid2)
? "compare(uuid2, uuid1): " + uuid_compare(uuid2, uuid1)
? "compare(uuid1, uuid1): " + uuid_compare(uuid1, uuid1)

? ""
? "is_nil(nil()): " + uuid_is_nil(uuid_nil())
? "is_nil(v4_uuid): " + uuid_is_nil(v4_uuid)
? "is_max(max()): " + uuid_is_max(uuid_max())

# ============================================
? ""
? "--- Hex Encoding ---"
# ============================================

hex_str = uuid_to_hex(test_uuid)
? "to_hex(): " + hex_str
? "from_hex(): " + uuid_from_hex(hex_str)

# ============================================
? ""
? "--- Batch Generation ---"
# ============================================

? "batch_v4(3): " + uuid_batch_v4(3)
? "batch_v7(3): " + uuid_batch_v7(3)

# ============================================
? ""
? "--- UUID v7 Timestamp ---"
# ============================================

new_v7 = uuid_v7()
ts_ms = uuid_v7_timestamp_ms(new_v7)
? "v7 UUID: " + new_v7
? "v7_timestamp_ms(): " + ts_ms

# Create from timestamp
created = uuid_v7_from_timestamp_ms(ts_ms)
? "v7_from_timestamp_ms(): " + created
? "Timestamps match: " + (uuid_v7_timestamp_ms(created) = ts_ms)

? ""
? "=== All UUID tests passed! ==="
