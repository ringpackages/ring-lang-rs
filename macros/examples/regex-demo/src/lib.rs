use regex::Regex;
use ring_lang_codegen::ring_extension;
use ring_lang_rs::*;

ring_extension! {
    prefix: "rx";

    /// Check if pattern is a valid regex
    pub fn is_valid(pattern: &str) -> bool {
        Regex::new(pattern).is_ok()
    }

    /// Check if text matches the pattern
    pub fn is_match(pattern: &str, text: &str) -> bool {
        match Regex::new(pattern) {
            Ok(re) => re.is_match(text),
            Err(_) => false,
        }
    }

    /// Find first match and return it (empty string if no match)
    pub fn find(pattern: &str, text: &str) -> String {
        match Regex::new(pattern) {
            Ok(re) => re.find(text)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default(),
            Err(_) => String::new(),
        }
    }

    /// Find all matches and return as JSON array
    pub fn find_all(pattern: &str, text: &str) -> String {
        match Regex::new(pattern) {
            Ok(re) => {
                let matches: Vec<&str> = re.find_iter(text)
                    .map(|m| m.as_str())
                    .collect();
                serde_json_mini(&matches)
            }
            Err(_) => "[]".to_string(),
        }
    }

    /// Count number of matches
    pub fn count(pattern: &str, text: &str) -> i64 {
        match Regex::new(pattern) {
            Ok(re) => re.find_iter(text).count() as i64,
            Err(_) => 0,
        }
    }

    /// Replace first match with replacement
    pub fn replace(pattern: &str, text: &str, replacement: &str) -> String {
        match Regex::new(pattern) {
            Ok(re) => re.replace(text, replacement).to_string(),
            Err(_) => text.to_string(),
        }
    }

    /// Replace all matches with replacement
    pub fn replace_all(pattern: &str, text: &str, replacement: &str) -> String {
        match Regex::new(pattern) {
            Ok(re) => re.replace_all(text, replacement).to_string(),
            Err(_) => text.to_string(),
        }
    }

    /// Split text by pattern and return as JSON array
    pub fn split(pattern: &str, text: &str) -> String {
        match Regex::new(pattern) {
            Ok(re) => {
                let parts: Vec<&str> = re.split(text).collect();
                serde_json_mini(&parts)
            }
            Err(_) => "[]".to_string(),
        }
    }

    /// Get start position of first match (-1 if no match)
    pub fn find_pos(pattern: &str, text: &str) -> i64 {
        match Regex::new(pattern) {
            Ok(re) => re.find(text)
                .map(|m| m.start() as i64)
                .unwrap_or(-1),
            Err(_) => -1,
        }
    }

    /// Get capture group from first match (group 0 = whole match)
    pub fn capture(pattern: &str, text: &str, group: i64) -> String {
        match Regex::new(pattern) {
            Ok(re) => re.captures(text)
                .and_then(|caps| caps.get(group as usize))
                .map(|m| m.as_str().to_string())
                .unwrap_or_default(),
            Err(_) => String::new(),
        }
    }

    /// Get all capture groups from first match as JSON array
    pub fn captures(pattern: &str, text: &str) -> String {
        match Regex::new(pattern) {
            Ok(re) => {
                match re.captures(text) {
                    Some(caps) => {
                        let groups: Vec<&str> = caps.iter()
                            .filter_map(|m| m.map(|m| m.as_str()))
                            .collect();
                        serde_json_mini(&groups)
                    }
                    None => "[]".to_string(),
                }
            }
            Err(_) => "[]".to_string(),
        }
    }

    /// Escape special regex characters in a string
    pub fn escape(text: &str) -> String {
        regex::escape(text)
    }

    // ============================================
    // Common Pattern Helpers
    // ============================================

    /// Check if string is a valid email (simple check)
    pub fn is_email(text: &str) -> bool {
        match Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$") {
            Ok(re) => re.is_match(text),
            Err(_) => false,
        }
    }

    /// Check if string is a valid URL
    pub fn is_url(text: &str) -> bool {
        match Regex::new(r"^https?://[^\s/$.?#].[^\s]*$") {
            Ok(re) => re.is_match(text),
            Err(_) => false,
        }
    }

    /// Check if string contains only digits
    pub fn is_digits(text: &str) -> bool {
        match Regex::new(r"^\d+$") {
            Ok(re) => re.is_match(text),
            Err(_) => false,
        }
    }

    /// Check if string is alphanumeric
    pub fn is_alphanumeric(text: &str) -> bool {
        match Regex::new(r"^[a-zA-Z0-9]+$") {
            Ok(re) => re.is_match(text),
            Err(_) => false,
        }
    }

    /// Extract all numbers from text as JSON array
    pub fn extract_numbers(text: &str) -> String {
        match Regex::new(r"-?\d+\.?\d*") {
            Ok(re) => {
                let nums: Vec<&str> = re.find_iter(text)
                    .map(|m| m.as_str())
                    .collect();
                serde_json_mini(&nums)
            }
            Err(_) => "[]".to_string(),
        }
    }

    /// Extract all words from text as JSON array
    pub fn extract_words(text: &str) -> String {
        match Regex::new(r"\b\w+\b") {
            Ok(re) => {
                let words: Vec<&str> = re.find_iter(text)
                    .map(|m| m.as_str())
                    .collect();
                serde_json_mini(&words)
            }
            Err(_) => "[]".to_string(),
        }
    }
}

// Simple JSON array serialization (avoid serde_json dependency)
fn serde_json_mini(items: &[&str]) -> String {
    let escaped: Vec<String> = items
        .iter()
        .map(|s| format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")))
        .collect();
    format!("[{}]", escaped.join(","))
}
