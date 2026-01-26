? "=== UUID Demo (ring_extension!) ==="

if iswindows()
    loadlib("target/release/ring_uuid.dll")
elseif ismacosx()
    loadlib("target/release/libring_uuid.dylib")
else
    loadlib("target/release/libring_uuid.so")
ok

? "v4: " + uuid_v4()
? "v7: " + uuid_v7()
? "nil: " + uuid_nil()
? "max: " + uuid_max()
? ""

cUuid = uuid_v4()
? "Generated: " + cUuid
? "is_valid: " + uuid_is_valid(cUuid)
? "version: " + uuid_version(cUuid)
? "to_upper: " + uuid_to_upper(cUuid)
? "to_simple: " + uuid_to_simple(cUuid)
? "to_urn: " + uuid_to_urn(cUuid)
? ""

? "is_valid('not-a-uuid'): " + uuid_is_valid("not-a-uuid")
? "is_nil(nil): " + uuid_is_nil(uuid_nil())
? "is_max(max): " + uuid_is_max(uuid_max())
? ""

u1 = uuid_v4()
u2 = uuid_v4()
? "equals(u1, u1): " + uuid_equals(u1, u1)
? "equals(u1, u2): " + uuid_equals(u1, u2)
? ""
? "Done!"
