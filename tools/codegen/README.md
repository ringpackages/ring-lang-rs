# Ring Rust Codegen

Generate Ring bindings for Rust functions, structs, and impl blocks automatically.

## Usage

```bash
ring parsec.ring input.rf output.rs [output.ring]
```

**Arguments:**
- `input.rf` - Configuration file with Rust function/struct definitions
- `output.rs` - Generated Rust source file
- `output.ring` - (Optional) Generated Ring class wrappers

## Quick Start

The simplest way to wrap Rust code for Ring:

```
<meta>
lib_prefix: my
</meta>

<code>
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
</code>
```

That's it! Functions with `pub fn` in `<code>` blocks are **auto-detected** and wrapped automatically. No separate `<functions>` block needed.

## Configuration File Format (.rf)

### Metadata Section

```
<meta>
crate_name: my_crate
lib_prefix: my
</meta>
```

- `crate_name` - The Rust crate to import (adds `use crate_name::*;`)
- `lib_prefix` - Prefix for all generated Ring function names

### Code Section (with Auto-Detection)

Include raw Rust code. Functions marked `pub fn` are automatically detected and wrapped:

```
<code>
use some_crate::SomeType;

// This function will be auto-wrapped for Ring
pub fn my_function(x: i32) -> i32 {
    x * 2
}

// This will also be auto-wrapped
pub fn process(input: &str) -> String {
    input.to_uppercase()
}

// NOT auto-wrapped (no pub)
fn internal_helper() -> i32 {
    42
}
</code>
```

**Auto-detection rules:**
- `pub fn name(...) -> Type { }` - Wrapped as standalone function
- `pub fn method(&self, ...) -> Type` - Skipped (impl method)
- `pub fn method(&mut self, ...)` - Skipped (impl method)
- `pub fn new(...) -> Self` - Skipped (constructor)

Indentation is preserved in code blocks.

### Functions Section (Optional)

Explicitly declare functions to wrap. Use this when:
- Functions are defined elsewhere (external crate)
- You want to override auto-detected signatures
- You need more control

```
<functions>
fn hello() -> String
fn add(a: i32, b: i32) -> i32
fn greet(name: &str) -> String
fn safe_divide(a: f64, b: f64) -> Result<f64, String>
fn find_item(id: i32) -> Option<String>
</functions>
```

**Supported return types:**
- Primitives: `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`, `f32`, `f64`, `bool`
- Strings: `String`, `&str`
- `Result<T, E>` - Returns T on success, raises Ring error on failure
- `Option<T>` - Returns T on Some, empty string on None
- Pointers: `*mut T`, `*const T`, `&T`, `&mut T`

### Struct Section

Define structs with auto-generated accessors:

```
<struct>
Config {
    pub timeout: u32,
    pub host: String,
    pub port: u16,
    pub enabled: bool,
}
</struct>
```

This generates:
- `prefix_structname_new()` - Constructor (or custom if impl has `new`)
- `prefix_structname_delete(ptr)` - Destructor
- `prefix_structname_get_field(ptr)` - Getter for each field
- `prefix_structname_set_field(ptr, value)` - Setter for each field

### Impl Section

Define methods for structs:

```
<impl>
impl Config {
    pub fn new(host: &str, port: u16) -> Self
    pub fn connect(&self) -> bool
    pub fn disconnect(&self)
    pub fn set_timeout(&mut self, timeout: u32)
}
</impl>
```

- Methods with `&self` or `&mut self` are instance methods
- `pub fn new(...) -> Self` replaces the default constructor
- Field accessors are skipped if impl has a method with the same name

### Constants Section

Expose constants:

```
<constants>
DEFAULT_TIMEOUT: u32
MAX_CONNECTIONS: u32
VERSION: &str
PI: f64
DEBUG_MODE: bool
</constants>
```

Generates getter functions: `prefix_get_constantname()`

### Comments

```
<comment>
This text is ignored by the parser
</comment>
```

### Conditional Sections

```
<filter> iswindows()
fn windows_only_function() -> i32
</filter>
```

### Include Files

```
<loadfile> other_definitions.rf
```

### Run Code During Parsing

```
<runcodenow> $globals + ["custom_value"]
```

### Manual Function Registration

For functions written directly as `ring_func!` macros (not auto-generated), register them manually:

```
<register>
encode
decode
my_custom_func => ring_prefix_my_custom_func
</register>
```

Each line can be:
- `name` - Auto-expands to `prefix_name => ring_prefix_name`
- `ring_name => rust_func_name` - Explicit mapping

## Examples

### Example 1: Wrapping External Crates

```
<meta>
lib_prefix: hash
</meta>

<code>
use base64::{engine::general_purpose::STANDARD, Engine};
use sha2::{Sha256, Digest};

pub fn base64_encode(input: &str) -> String {
    STANDARD.encode(input.as_bytes())
}

pub fn sha256_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}
</code>

# No <functions> block needed - pub fn auto-detected!
```

### Example 2: Struct with Custom Constructor

```
<meta>
lib_prefix: geo
</meta>

<code>
#[derive(Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}
</code>

<struct>
Point {
    pub x: f64,
    pub y: f64,
}
</struct>

<impl>
impl Point {
    pub fn new(x: f64, y: f64) -> Self
    pub fn distance(&self, other: &Point) -> f64
}
</impl>
```

### Example 3: Using External Crate Functions

```
<meta>
crate_name: mylib
lib_prefix: ml
</meta>

<functions>
fn add(a: i32, b: i32) -> i32
fn greet(name: &str) -> String
</functions>
```

## Generated Ring Usage

```ring
loadlib("libmylib.so")  # or .dll/.dylib

# Functions
? ml_add(10, 20)        # 30
? ml_greet("World")     # Hello, World!

# Struct via low-level API
ptr = ml_point_new(10.5, 20.5)
? ml_point_get_x(ptr)   # 10.5
ml_point_set_x(ptr, 15.0)
? ml_point_distance(ptr, other_ptr)
ml_point_delete(ptr)

# Struct via Ring class (if output.ring generated)
load "output.ring"
p = new Point(10.5, 20.5)
? p.x()                 # 10.5
p.setX(15.0)
p.delete()
```

## Files

- `parsec.ring` - Main codegen tool
- `gendoc.ring` - Documentation generator (creates API.md from .rf files)
- `examples/hash-demo/` - Hash library example (base64, md5, sha256, sha512)
- `examples/datetime-demo/` - DateTime library example (chrono wrapper)
- `examples/uuid-demo/` - UUID library example (uuid wrapper)
- `examples/json-demo/` - JSON library example (serde_json wrapper)
