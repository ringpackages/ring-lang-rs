use ring_lang_codegen::ring_extension;
use ring_lang_rs::*;
use uuid::Uuid;

ring_extension! {
    prefix: "uuid";

    /// Generate a new UUID v4 (random)
    pub fn v4() -> String {
        Uuid::new_v4().to_string()
    }

    /// Generate a new UUID v7 (time-ordered)
    pub fn v7() -> String {
        Uuid::now_v7().to_string()
    }

    /// Generate a nil UUID (all zeros)
    pub fn nil() -> String {
        Uuid::nil().to_string()
    }

    /// Generate a max UUID (all ones)
    pub fn max() -> String {
        Uuid::max().to_string()
    }

    /// Check if a string is a valid UUID
    pub fn is_valid(uuid_str: &str) -> bool {
        Uuid::parse_str(uuid_str).is_ok()
    }

    /// Parse and normalize a UUID string
    pub fn parse(uuid_str: &str) -> String {
        match Uuid::parse_str(uuid_str) {
            Ok(uuid) => uuid.to_string(),
            Err(_) => String::new(),
        }
    }

    /// Get UUID version (returns 0 if invalid)
    pub fn version(uuid_str: &str) -> i32 {
        match Uuid::parse_str(uuid_str) {
            Ok(uuid) => uuid.get_version_num() as i32,
            Err(_) => 0,
        }
    }

    /// Check if UUID is nil (all zeros)
    pub fn is_nil(uuid_str: &str) -> bool {
        match Uuid::parse_str(uuid_str) {
            Ok(uuid) => uuid.is_nil(),
            Err(_) => false,
        }
    }

    /// Check if UUID is max (all ones)
    pub fn is_max(uuid_str: &str) -> bool {
        match Uuid::parse_str(uuid_str) {
            Ok(uuid) => uuid.is_max(),
            Err(_) => false,
        }
    }

    /// Convert UUID to uppercase
    pub fn to_upper(uuid_str: &str) -> String {
        match Uuid::parse_str(uuid_str) {
            Ok(uuid) => uuid.to_string().to_uppercase(),
            Err(_) => String::new(),
        }
    }

    /// Convert UUID to URN format
    pub fn to_urn(uuid_str: &str) -> String {
        match Uuid::parse_str(uuid_str) {
            Ok(uuid) => uuid.urn().to_string(),
            Err(_) => String::new(),
        }
    }

    /// Convert UUID to simple format (no hyphens)
    pub fn to_simple(uuid_str: &str) -> String {
        match Uuid::parse_str(uuid_str) {
            Ok(uuid) => uuid.simple().to_string(),
            Err(_) => String::new(),
        }
    }

    /// Compare two UUIDs (-1, 0, 1)
    pub fn compare(uuid1: &str, uuid2: &str) -> i32 {
        let u1 = Uuid::parse_str(uuid1);
        let u2 = Uuid::parse_str(uuid2);

        match (u1, u2) {
            (Ok(a), Ok(b)) => a.cmp(&b) as i32,
            _ => 0,
        }
    }

    /// Check if two UUIDs are equal
    pub fn equals(uuid1: &str, uuid2: &str) -> bool {
        let u1 = Uuid::parse_str(uuid1);
        let u2 = Uuid::parse_str(uuid2);

        match (u1, u2) {
            (Ok(a), Ok(b)) => a == b,
            _ => false,
        }
    }
}
