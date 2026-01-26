? "=== JSON Demo (ring_extension!) ==="

if iswindows()
    loadlib("target/release/ring_json.dll")
elseif ismacosx()
    loadlib("target/release/libring_json.dylib")
else
    loadlib("target/release/libring_json.so")
ok

cJson = '{"name":"John","age":30,"active":true,"tags":["dev","rust"]}'

? "is_valid: " + json_is_valid(cJson)
? "is_valid (bad): " + json_is_valid("not json")
? ""
? "prettify:"
? json_prettify(cJson)
? ""
? "get_string('name'): " + json_get_string(cJson, "name")
? "get_number('age'): " + json_get_number(cJson, "age")
? "get_bool('active'): " + json_get_bool(cJson, "active")
? "get_type('tags'): " + json_get_type(cJson, "tags")
? "has_path('name'): " + json_has_path(cJson, "name")
? "has_path('missing'): " + json_has_path(cJson, "missing")
? ""
? "array_len('tags'): " + json_array_len(cJson, "tags")
? "object_keys(''): " + json_object_keys(cJson, "")
? ""
? "set_string: " + json_set_string(cJson, "name", "Jane")
? "merge: " + json_merge('{"a":1}', '{"b":2}')
? ""
? "Done!"
