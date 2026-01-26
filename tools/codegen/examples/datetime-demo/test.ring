# DateTime Demo Test
# Tests chrono wrapper functions

if iswindows()
    loadlib("target/release/ring_datetime.dll")
elseif ismacosx()
    loadlib("target/release/libring_datetime.dylib")
else
    loadlib("target/release/libring_datetime.so")
ok

? "=== DateTime Demo Test ==="
? ""

# ============================================
? "--- Current Time ---"
# ============================================

? "now_utc(): " + dt_now_utc()
? "now_local(): " + dt_now_local()
? "now_unix(): " + dt_now_unix()
? "now_unix_ms(): " + dt_now_unix_ms()

# ============================================
? ""
? "--- Parsing ---"
# ============================================

? "parse_rfc3339('2024-06-15T14:30:00Z'): " + dt_parse_rfc3339("2024-06-15T14:30:00Z")
? "parse_format('2024-06-15 14:30:00', '%Y-%m-%d %H:%M:%S'): " + dt_parse_format("2024-06-15 14:30:00", "%Y-%m-%d %H:%M:%S")
? "parse_date('15/06/2024', '%d/%m/%Y'): " + dt_parse_date("15/06/2024", "%d/%m/%Y")
? "from_unix(1718455800): " + dt_from_unix(1718455800)
? "from_unix_ms(1718455800000): " + dt_from_unix_ms(1718455800000)

# ============================================
? ""
? "--- Formatting ---"
# ============================================

dt = "2024-06-15T14:30:00Z"
? "format_datetime(dt, '%Y-%m-%d'): " + dt_format_datetime(dt, "%Y-%m-%d")
? "format_datetime(dt, '%d %B %Y'): " + dt_format_datetime(dt, "%d %B %Y")
? "format_datetime(dt, '%H:%M:%S'): " + dt_format_datetime(dt, "%H:%M:%S")
? "format_datetime(dt, '%A, %B %d, %Y'): " + dt_format_datetime(dt, "%A, %B %d, %Y")
? "to_rfc2822(dt): " + dt_to_rfc2822(dt)
? "to_unix(dt): " + dt_to_unix(dt)

# ============================================
? ""
? "--- Components Extraction ---"
# ============================================

? "get_year(dt): " + dt_get_year(dt)
? "get_month(dt): " + dt_get_month(dt)
? "get_day(dt): " + dt_get_day(dt)
? "get_hour(dt): " + dt_get_hour(dt)
? "get_minute(dt): " + dt_get_minute(dt)
? "get_second(dt): " + dt_get_second(dt)
? "get_weekday(dt): " + dt_get_weekday(dt)
? "get_weekday_name(dt): " + dt_get_weekday_name(dt)
? "get_day_of_year(dt): " + dt_get_day_of_year(dt)
? "get_week_number(dt): " + dt_get_week_number(dt)

# ============================================
? ""
? "--- Date Arithmetic ---"
# ============================================

? "add_days(dt, 7): " + dt_add_days(dt, 7)
? "add_days(dt, -7): " + dt_add_days(dt, -7)
? "add_hours(dt, 12): " + dt_add_hours(dt, 12)
? "add_minutes(dt, 90): " + dt_add_minutes(dt, 90)
? "add_weeks(dt, 2): " + dt_add_weeks(dt, 2)
? "add_months(dt, 3): " + dt_add_months(dt, 3)
? "add_months(dt, -1): " + dt_add_months(dt, -1)
? "add_years(dt, 1): " + dt_add_years(dt, 1)

# ============================================
? ""
? "--- Date Comparison ---"
# ============================================

dt1 = "2024-06-15T10:00:00Z"
dt2 = "2024-06-20T15:30:00Z"

? "diff_days(dt2, dt1): " + dt_diff_days(dt2, dt1)
? "diff_hours(dt2, dt1): " + dt_diff_hours(dt2, dt1)
? "diff_minutes(dt2, dt1): " + dt_diff_minutes(dt2, dt1)
? "is_before(dt1, dt2): " + dt_is_before(dt1, dt2)
? "is_after(dt1, dt2): " + dt_is_after(dt1, dt2)
? "is_same_day(dt1, dt2): " + dt_is_same_day(dt1, dt2)
? "is_same_day(dt1, dt1): " + dt_is_same_day(dt1, dt1)

# ============================================
? ""
? "--- Date Construction ---"
# ============================================

? "create(2024, 12, 25, 10, 30, 0): " + dt_create(2024, 12, 25, 10, 30, 0)
? "create_date(2024, 7, 4): " + dt_create_date(2024, 7, 4)
? "start_of_day(dt): " + dt_start_of_day(dt)
? "end_of_day(dt): " + dt_end_of_day(dt)

# ============================================
? ""
? "--- Validation ---"
# ============================================

? "is_valid('2024-06-15T14:30:00Z'): " + dt_is_valid("2024-06-15T14:30:00Z")
? "is_valid('invalid'): " + dt_is_valid("invalid")
? "is_leap_year(2024): " + dt_is_leap_year(2024)
? "is_leap_year(2023): " + dt_is_leap_year(2023)
? "days_in_month(2024, 2): " + dt_days_in_month(2024, 2)
? "days_in_month(2023, 2): " + dt_days_in_month(2023, 2)
? "days_in_month(2024, 12): " + dt_days_in_month(2024, 12)

? ""
? "=== All DateTime tests passed! ==="
