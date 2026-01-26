? "=== Regex Demo (ring_extension!) ==="

if iswindows()
    loadlib("target/release/ring_regex.dll")
elseif ismacosx()
    loadlib("target/release/libring_regex.dylib")
else
    loadlib("target/release/libring_regex.so")
ok

cText = "Hello World! Contact us at test@example.com or visit https://example.com"

? "is_valid('[a-z]+'): " + rx_is_valid("[a-z]+")
? "is_valid('[invalid'): " + rx_is_valid("[invalid")
? ""

? "is_match('\\w+', 'hello'): " + rx_is_match("\w+", "hello")
? "find('[a-z]+', text): " + rx_find("[a-z]+", cText)
? "find_all('[A-Z][a-z]+', text): " + rx_find_all("[A-Z][a-z]+", cText)
? "count('\\w+', text): " + rx_count("\w+", cText)
? ""

? "replace('World', text, 'Ring'): " + rx_replace("World", cText, "Ring")
? "replace_all('[aeiou]', 'hello', '*'): " + rx_replace_all("[aeiou]", "hello", "*")
? ""

? "split('\\s+', 'a b  c'): " + rx_split("\s+", "a b  c")
? "find_pos('World', text): " + rx_find_pos("World", cText)
? ""

? "capture('(\\w+)@(\\w+)', text, 0): " + rx_capture("(\w+)@(\w+)", cText, 0)
? "capture('(\\w+)@(\\w+)', text, 1): " + rx_capture("(\w+)@(\w+)", cText, 1)
? "captures('(\\w+)@(\\w+)', text): " + rx_captures("(\w+)@(\w+)", cText)
? ""

? "escape('a.b*c?'): " + rx_escape("a.b*c?")
? ""

? "is_email('test@example.com'): " + rx_is_email("test@example.com")
? "is_email('invalid'): " + rx_is_email("invalid")
? "is_url('https://example.com'): " + rx_is_url("https://example.com")
? "is_digits('12345'): " + rx_is_digits("12345")
? "is_alphanumeric('abc123'): " + rx_is_alphanumeric("abc123")
? ""

? "extract_numbers('Price: $19.99, Qty: 5'): " + rx_extract_numbers("Price: $19.99, Qty: 5")
? "extract_words('Hello, World!'): " + rx_extract_words("Hello, World!")
? ""
? "Done!"
