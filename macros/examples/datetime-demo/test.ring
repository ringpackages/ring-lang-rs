? "=== DateTime Demo (ring_extension!) ==="

if iswindows()
    loadlib("target/release/ring_datetime.dll")
elseif ismacosx()
    loadlib("target/release/libring_datetime.dylib")
else
    loadlib("target/release/libring_datetime.so")
ok

? "now_utc: " + dt_now_utc()
? "now_local: " + dt_now_local()
? "unix_timestamp: " + dt_unix_timestamp()
? "unix_millis: " + dt_unix_millis()
? ""

cNow = dt_now_utc()
? "Current: " + cNow
? "year: " + dt_year(cNow)
? "month: " + dt_month(cNow)
? "day: " + dt_day(cNow)
? "hour: " + dt_hour(cNow)
? "minute: " + dt_minute(cNow)
? "second: " + dt_second(cNow)
? "weekday: " + dt_weekday(cNow)
? "day_of_year: " + dt_day_of_year(cNow)
? "week_number: " + dt_week_number(cNow)
? ""

? "to_date: " + dt_to_date(cNow)
? "to_time: " + dt_to_time(cNow)
? "format_dt: " + dt_format_dt(cNow, "%Y/%m/%d %H:%M")
? ""

? "add_days(+7): " + dt_add_days(cNow, 7)
? "add_hours(-2): " + dt_add_hours(cNow, -2)
? ""

cPast = dt_add_days(cNow, -10)
? "diff_days: " + dt_diff_days(cNow, cPast)
? "is_before: " + dt_is_before(cPast, cNow)
? "is_after: " + dt_is_after(cNow, cPast)
? ""

? "from_unix(0): " + dt_from_unix(0)
? "parse_date: " + dt_parse_date("2025-01-26")
? "is_valid: " + dt_is_valid(cNow)
? ""
? "Done!"
