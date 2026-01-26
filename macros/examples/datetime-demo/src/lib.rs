use chrono::{
    DateTime, Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc,
};
use ring_lang_codegen::ring_extension;
use ring_lang_rs::*;

ring_extension! {
    prefix: "dt";

    // ============================================
    // Current Time Functions
    // ============================================

    /// Get current UTC timestamp as ISO 8601 string
    pub fn now_utc() -> String {
        Utc::now().to_rfc3339()
    }

    /// Get current local timestamp as ISO 8601 string
    pub fn now_local() -> String {
        Local::now().to_rfc3339()
    }

    /// Get current Unix timestamp (seconds since epoch)
    pub fn unix_timestamp() -> i64 {
        Utc::now().timestamp()
    }

    /// Get current Unix timestamp in milliseconds
    pub fn unix_millis() -> i64 {
        Utc::now().timestamp_millis()
    }

    // ============================================
    // Parsing Functions
    // ============================================

    /// Parse ISO 8601 / RFC 3339 datetime string
    pub fn parse_iso(datetime_str: &str) -> String {
        match DateTime::parse_from_rfc3339(datetime_str) {
            Ok(dt) => dt.to_rfc3339(),
            Err(_) => String::new(),
        }
    }

    /// Parse datetime with custom format
    pub fn parse_format(datetime_str: &str, format: &str) -> String {
        match NaiveDateTime::parse_from_str(datetime_str, format) {
            Ok(dt) => dt.format("%Y-%m-%dT%H:%M:%S").to_string(),
            Err(_) => String::new(),
        }
    }

    /// Parse date string (YYYY-MM-DD)
    pub fn parse_date(date_str: &str) -> String {
        match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            Ok(d) => d.to_string(),
            Err(_) => String::new(),
        }
    }

    /// Parse time string (HH:MM:SS)
    pub fn parse_time(time_str: &str) -> String {
        match NaiveTime::parse_from_str(time_str, "%H:%M:%S") {
            Ok(t) => t.to_string(),
            Err(_) => String::new(),
        }
    }

    /// Convert Unix timestamp to ISO string
    pub fn from_unix(timestamp: i64) -> String {
        match DateTime::from_timestamp(timestamp, 0) {
            Some(dt) => dt.to_rfc3339(),
            None => String::new(),
        }
    }

    /// Convert Unix milliseconds to ISO string
    pub fn from_unix_millis(millis: i64) -> String {
        match DateTime::from_timestamp_millis(millis) {
            Some(dt) => dt.to_rfc3339(),
            None => String::new(),
        }
    }

    // ============================================
    // Formatting Functions
    // ============================================

    /// Format datetime with custom format string
    pub fn format_dt(datetime_str: &str, fmt: &str) -> String {
        match DateTime::parse_from_rfc3339(datetime_str) {
            Ok(dt) => dt.format(fmt).to_string(),
            Err(_) => {
                match NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%dT%H:%M:%S") {
                    Ok(dt) => dt.format(fmt).to_string(),
                    Err(_) => String::new(),
                }
            }
        }
    }

    /// Get ISO 8601 date only (YYYY-MM-DD)
    pub fn to_date(datetime_str: &str) -> String {
        match DateTime::parse_from_rfc3339(datetime_str) {
            Ok(dt) => dt.format("%Y-%m-%d").to_string(),
            Err(_) => String::new(),
        }
    }

    /// Get time only (HH:MM:SS)
    pub fn to_time(datetime_str: &str) -> String {
        match DateTime::parse_from_rfc3339(datetime_str) {
            Ok(dt) => dt.format("%H:%M:%S").to_string(),
            Err(_) => String::new(),
        }
    }

    // ============================================
    // Component Extraction
    // ============================================

    /// Get year from datetime
    pub fn year(datetime_str: &str) -> i32 {
        match DateTime::parse_from_rfc3339(datetime_str) {
            Ok(dt) => dt.year(),
            Err(_) => 0,
        }
    }

    /// Get month (1-12) from datetime
    pub fn month(datetime_str: &str) -> i32 {
        match DateTime::parse_from_rfc3339(datetime_str) {
            Ok(dt) => dt.month() as i32,
            Err(_) => 0,
        }
    }

    /// Get day of month (1-31) from datetime
    pub fn day(datetime_str: &str) -> i32 {
        match DateTime::parse_from_rfc3339(datetime_str) {
            Ok(dt) => dt.day() as i32,
            Err(_) => 0,
        }
    }

    /// Get hour (0-23) from datetime
    pub fn hour(datetime_str: &str) -> i32 {
        match DateTime::parse_from_rfc3339(datetime_str) {
            Ok(dt) => dt.hour() as i32,
            Err(_) => 0,
        }
    }

    /// Get minute (0-59) from datetime
    pub fn minute(datetime_str: &str) -> i32 {
        match DateTime::parse_from_rfc3339(datetime_str) {
            Ok(dt) => dt.minute() as i32,
            Err(_) => 0,
        }
    }

    /// Get second (0-59) from datetime
    pub fn second(datetime_str: &str) -> i32 {
        match DateTime::parse_from_rfc3339(datetime_str) {
            Ok(dt) => dt.second() as i32,
            Err(_) => 0,
        }
    }

    /// Get day of week (0=Sunday, 6=Saturday)
    pub fn weekday(datetime_str: &str) -> i32 {
        match DateTime::parse_from_rfc3339(datetime_str) {
            Ok(dt) => dt.weekday().num_days_from_sunday() as i32,
            Err(_) => 0,
        }
    }

    /// Get day of year (1-366)
    pub fn day_of_year(datetime_str: &str) -> i32 {
        match DateTime::parse_from_rfc3339(datetime_str) {
            Ok(dt) => dt.ordinal() as i32,
            Err(_) => 0,
        }
    }

    /// Get ISO week number (1-53)
    pub fn week_number(datetime_str: &str) -> i32 {
        match DateTime::parse_from_rfc3339(datetime_str) {
            Ok(dt) => dt.iso_week().week() as i32,
            Err(_) => 0,
        }
    }

    // ============================================
    // Arithmetic Functions
    // ============================================

    /// Add days to datetime
    pub fn add_days(datetime_str: &str, days: i64) -> String {
        match DateTime::parse_from_rfc3339(datetime_str) {
            Ok(dt) => (dt + Duration::days(days)).to_rfc3339(),
            Err(_) => String::new(),
        }
    }

    /// Add hours to datetime
    pub fn add_hours(datetime_str: &str, hours: i64) -> String {
        match DateTime::parse_from_rfc3339(datetime_str) {
            Ok(dt) => (dt + Duration::hours(hours)).to_rfc3339(),
            Err(_) => String::new(),
        }
    }

    /// Add minutes to datetime
    pub fn add_minutes(datetime_str: &str, minutes: i64) -> String {
        match DateTime::parse_from_rfc3339(datetime_str) {
            Ok(dt) => (dt + Duration::minutes(minutes)).to_rfc3339(),
            Err(_) => String::new(),
        }
    }

    /// Add seconds to datetime
    pub fn add_seconds(datetime_str: &str, seconds: i64) -> String {
        match DateTime::parse_from_rfc3339(datetime_str) {
            Ok(dt) => (dt + Duration::seconds(seconds)).to_rfc3339(),
            Err(_) => String::new(),
        }
    }

    /// Difference between two datetimes in seconds
    pub fn diff_seconds(dt1: &str, dt2: &str) -> i64 {
        let d1 = DateTime::parse_from_rfc3339(dt1);
        let d2 = DateTime::parse_from_rfc3339(dt2);

        match (d1, d2) {
            (Ok(a), Ok(b)) => (a - b).num_seconds(),
            _ => 0,
        }
    }

    /// Difference between two datetimes in days
    pub fn diff_days(dt1: &str, dt2: &str) -> i64 {
        let d1 = DateTime::parse_from_rfc3339(dt1);
        let d2 = DateTime::parse_from_rfc3339(dt2);

        match (d1, d2) {
            (Ok(a), Ok(b)) => (a - b).num_days(),
            _ => 0,
        }
    }

    // ============================================
    // Comparison Functions
    // ============================================

    /// Check if dt1 is before dt2
    pub fn is_before(dt1: &str, dt2: &str) -> bool {
        let d1 = DateTime::parse_from_rfc3339(dt1);
        let d2 = DateTime::parse_from_rfc3339(dt2);

        match (d1, d2) {
            (Ok(a), Ok(b)) => a < b,
            _ => false,
        }
    }

    /// Check if dt1 is after dt2
    pub fn is_after(dt1: &str, dt2: &str) -> bool {
        let d1 = DateTime::parse_from_rfc3339(dt1);
        let d2 = DateTime::parse_from_rfc3339(dt2);

        match (d1, d2) {
            (Ok(a), Ok(b)) => a > b,
            _ => false,
        }
    }

    /// Check if datetime is valid
    pub fn is_valid(datetime_str: &str) -> bool {
        DateTime::parse_from_rfc3339(datetime_str).is_ok()
    }
}
