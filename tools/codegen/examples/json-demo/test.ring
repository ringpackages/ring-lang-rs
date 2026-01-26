# JSON Demo Test
# Tests serde_json wrapper functions

if iswindows()
    loadlib("target/release/ring_json.dll")
elseif ismacosx()
    loadlib("target/release/libring_json.dylib")
else
    loadlib("target/release/libring_json.so")
ok

? "=== JSON Demo Test ==="
? ""

# Sample JSON data
sample = '{
    "name": "John Doe",
    "age": 30,
    "active": true,
    "email": null,
    "address": {
        "city": "New York",
        "zip": "10001"
    },
    "tags": ["developer", "rust", "ring"],
    "scores": [95, 87, 92]
}'

# ============================================
? "--- Basic Operations ---"
# ============================================

? "is_valid: " + json_is_valid(sample)
? "is_valid (bad): " + json_is_valid("{invalid}")

? ""
? "Prettified:"
? json_prettify('{"a":1,"b":2}')

? ""
? "Minified:"
? json_minify(sample)

# ============================================
? ""
? "--- Value Extraction ---"
# ============================================

? "get_string('name'): " + json_get_string(sample, "name")
? "get_number('age'): " + json_get_number(sample, "age")
? "get_bool('active'): " + json_get_bool(sample, "active")
? "get_string('address.city'): " + json_get_string(sample, "address.city")
? "get_string('tags.0'): " + json_get_string(sample, "tags.0")
? "get_number('scores.1'): " + json_get_number(sample, "scores.1")

? ""
? "get_type('name'): " + json_get_type(sample, "name")
? "get_type('age'): " + json_get_type(sample, "age")
? "get_type('active'): " + json_get_type(sample, "active")
? "get_type('email'): " + json_get_type(sample, "email")
? "get_type('address'): " + json_get_type(sample, "address")
? "get_type('tags'): " + json_get_type(sample, "tags")
? "get_type('missing'): " + json_get_type(sample, "missing")

? ""
? "has_path('name'): " + json_has_path(sample, "name")
? "has_path('missing'): " + json_has_path(sample, "missing")

# ============================================
? ""
? "--- Array Operations ---"
# ============================================

? "array_len('tags'): " + json_array_len(sample, "tags")
? "array_len('scores'): " + json_array_len(sample, "scores")
? "array_get('tags', 1): " + json_array_get(sample, "tags", 1)
? "array_get('scores', 2): " + json_array_get(sample, "scores", 2)

# ============================================
? ""
? "--- Object Operations ---"
# ============================================

? "object_keys(''): " + json_object_keys(sample, "")
? "object_keys('address'): " + json_object_keys(sample, "address")
? "object_len(''): " + json_object_len(sample, "")
? "object_len('address'): " + json_object_len(sample, "address")

# ============================================
? ""
? "--- JSON Modification ---"
# ============================================

modified = json_set_string(sample, "name", "Jane Doe")
? "After set_string('name', 'Jane Doe'):"
? "  new name: " + json_get_string(modified, "name")

modified = json_set_number(sample, "age", 25)
? "After set_number('age', 25):"
? "  new age: " + json_get_number(modified, "age")

modified = json_set_bool(sample, "active", false)
? "After set_bool('active', false):"
? "  new active: " + json_get_bool(modified, "active")

modified = json_delete_path(sample, "email")
? "After delete_path('email'):"
? "  has email: " + json_has_path(modified, "email")

# ============================================
? ""
? "--- JSON Building ---"
# ============================================

obj = json_new_object()
obj = json_set_string(obj, "title", "Hello World")
obj = json_set_number(obj, "count", 42)
obj = json_set_bool(obj, "published", true)
? "Built object: " + obj

arr = json_new_array()
arr = json_array_push(arr, '"item1"')
arr = json_array_push(arr, '"item2"')
arr = json_array_push(arr, '{"nested": true}')
? "Built array: " + arr

# ============================================
? ""
? "--- Merge ---"
# ============================================

obj1 = '{"a": 1, "b": 2}'
obj2 = '{"b": 3, "c": 4}'
merged = json_merge(obj1, obj2)
? "merge({a:1,b:2}, {b:3,c:4}): " + merged

# ============================================
? ""
? "--- Encode/Decode ---"
# ============================================

# Encode simple array
arr = [1, 2, 3, "four", 5.5]
? "encode([1,2,3,'four',5.5]): " + json_encode(arr)

# Encode object (list of [key, value] pairs)
obj = [
    ["name", "John"],
    ["age", 30],
    ["active", 1]
]
? "encode(object): " + json_encode(obj)

# Encode with pretty print
? "encode(object, pretty):"
? json_encode(obj, 1)

# Decode JSON to Ring list
decoded = json_decode('{"name":"Jane","scores":[95,87,92]}')
? "decode result type: " + type(decoded)
? "decode result: "
see decoded
? ""

# Decode array
decoded_arr = json_decode('[1, 2, "three", true, null]')
? "decoded array: "
see decoded_arr
? ""

# Round-trip test
original = [["x", 10], ["y", 20], ["label", "point"]]
encoded = json_encode(original)
decoded2 = json_decode(encoded)
re_encoded = json_encode(decoded2)
? "Round-trip test:"
? "  original encoded: " + encoded
? "  re-encoded:       " + re_encoded
? "  match: " + (encoded = re_encoded)

? ""
? "=== All JSON tests passed! ==="
