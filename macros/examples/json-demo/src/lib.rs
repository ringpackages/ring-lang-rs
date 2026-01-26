use ring_lang_codegen::ring_extension;
use ring_lang_rs::*;
use serde_json::Value;

ring_extension! {
    prefix: "json";

    /// Parse JSON and check if valid
    pub fn is_valid(json_str: &str) -> bool {
        serde_json::from_str::<Value>(json_str).is_ok()
    }

    /// Pretty print JSON with indentation
    pub fn prettify(json_str: &str) -> String {
        match serde_json::from_str::<Value>(json_str) {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap_or_default(),
            Err(e) => format!("error: {}", e),
        }
    }

    /// Minify JSON (remove whitespace)
    pub fn minify(json_str: &str) -> String {
        match serde_json::from_str::<Value>(json_str) {
            Ok(v) => serde_json::to_string(&v).unwrap_or_default(),
            Err(e) => format!("error: {}", e),
        }
    }

    /// Get a string value at path (e.g., "user.name" or "items.0.id")
    pub fn get_string(json_str: &str, path: &str) -> String {
        get_value_at_path(json_str, path)
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_default()
    }

    /// Get a number value at path
    pub fn get_number(json_str: &str, path: &str) -> f64 {
        get_value_at_path(json_str, path)
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0)
    }

    /// Get a boolean value at path
    pub fn get_bool(json_str: &str, path: &str) -> bool {
        get_value_at_path(json_str, path)
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Check if a path exists in JSON
    pub fn has_path(json_str: &str, path: &str) -> bool {
        get_value_at_path(json_str, path).is_some()
    }

    /// Get the type of value at path
    pub fn get_type(json_str: &str, path: &str) -> String {
        match get_value_at_path(json_str, path) {
            Some(v) => match v {
                Value::Null => "null",
                Value::Bool(_) => "boolean",
                Value::Number(_) => "number",
                Value::String(_) => "string",
                Value::Array(_) => "array",
                Value::Object(_) => "object",
            }.to_string(),
            None => "undefined".to_string(),
        }
    }

    /// Get array length at path
    pub fn array_len(json_str: &str, path: &str) -> i64 {
        get_value_at_path(json_str, path)
            .and_then(|v| v.as_array().map(|a| a.len() as i64))
            .unwrap_or(0)
    }

    /// Get all keys of an object at path
    pub fn object_keys(json_str: &str, path: &str) -> String {
        let value = if path.is_empty() {
            serde_json::from_str::<Value>(json_str).ok()
        } else {
            get_value_at_path(json_str, path)
        };

        value
            .and_then(|v| v.as_object().cloned())
            .map(|o| {
                let keys: Vec<&str> = o.keys().map(|s| s.as_str()).collect();
                serde_json::to_string(&keys).unwrap_or("[]".to_string())
            })
            .unwrap_or("[]".to_string())
    }

    /// Set a string value at path
    pub fn set_string(json_str: &str, path: &str, value: &str) -> String {
        set_value_at_path(json_str, path, Value::String(value.to_string()))
    }

    /// Set a number value at path
    pub fn set_number(json_str: &str, path: &str, value: f64) -> String {
        set_value_at_path(json_str, path, serde_json::Number::from_f64(value)
            .map(Value::Number)
            .unwrap_or(Value::Null))
    }

    /// Create an empty JSON object
    pub fn new_object() -> String {
        "{}".to_string()
    }

    /// Create an empty JSON array
    pub fn new_array() -> String {
        "[]".to_string()
    }

    /// Merge two JSON objects
    pub fn merge(json1: &str, json2: &str) -> String {
        let mut obj1 = serde_json::from_str::<Value>(json1)
            .ok()
            .and_then(|v| v.as_object().cloned())
            .unwrap_or_default();

        let obj2 = serde_json::from_str::<Value>(json2)
            .ok()
            .and_then(|v| v.as_object().cloned())
            .unwrap_or_default();

        for (k, v) in obj2 {
            obj1.insert(k, v);
        }

        serde_json::to_string(&Value::Object(obj1)).unwrap_or("{}".to_string())
    }
}

// Helper functions (not exposed to Ring)
fn get_value_at_path(json_str: &str, path: &str) -> Option<Value> {
    let root: Value = serde_json::from_str(json_str).ok()?;
    let parts: Vec<&str> = path.split('.').filter(|s| !s.is_empty()).collect();

    let mut current = &root;
    for part in parts {
        if let Ok(index) = part.parse::<usize>() {
            current = current.get(index)?;
        } else {
            current = current.get(part)?;
        }
    }
    Some(current.clone())
}

fn set_value_at_path(json_str: &str, path: &str, new_value: Value) -> String {
    let mut root: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(_) => return json_str.to_string(),
    };

    let parts: Vec<&str> = path.split('.').filter(|s| !s.is_empty()).collect();

    if parts.is_empty() {
        return serde_json::to_string(&new_value).unwrap_or_default();
    }

    let mut current = &mut root;
    for (i, part) in parts.iter().enumerate() {
        let is_last = i == parts.len() - 1;

        if is_last {
            if let Ok(index) = part.parse::<usize>() {
                if let Value::Array(arr) = current {
                    if index < arr.len() {
                        arr[index] = new_value.clone();
                    }
                }
            } else if let Value::Object(obj) = current {
                obj.insert(part.to_string(), new_value.clone());
            }
        } else {
            if let Ok(index) = part.parse::<usize>() {
                current = match current.get_mut(index) {
                    Some(v) => v,
                    None => return json_str.to_string(),
                };
            } else {
                current = match current.get_mut(*part) {
                    Some(v) => v,
                    None => return json_str.to_string(),
                };
            }
        }
    }

    serde_json::to_string(&root).unwrap_or(json_str.to_string())
}
