//! Calendar date without time zone, enough for publication dates.

use std::fmt;

/// English abbreviations used by RFC 2822 dates in RSS feeds.
const WEEKDAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// A validated Gregorian calendar date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

/// Why a date string was rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateError(pub String);

impl fmt::Display for DateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid date '{}', expected YYYY-MM-DD", self.0)
    }
}

impl std::error::Error for DateError {}

fn is_leap_year(year: u16) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        2 if is_leap_year(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

impl Date {
    /// Parses an ISO 8601 calendar date (`YYYY-MM-DD`).
    pub fn parse(value: &str) -> Result<Self, DateError> {
        let err = || DateError(value.to_string());
        let bytes = value.as_bytes();
        if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
            return Err(err());
        }
        let year: u16 = value[0..4].parse().map_err(|_| err())?;
        let month: u8 = value[5..7].parse().map_err(|_| err())?;
        let day: u8 = value[8..10].parse().map_err(|_| err())?;
        if !(1..=12).contains(&month) || day == 0 || day > days_in_month(year, month) {
            return Err(err());
        }
        Ok(Self { year, month, day })
    }

    /// `2026-01-05`
    pub fn iso(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// Finnish short format: `5.1.2026`
    pub fn finnish(&self) -> String {
        format!("{}.{}.{}", self.day, self.month, self.year)
    }

    /// Days since 1970-01-01 (Howard Hinnant's `days_from_civil`).
    fn days_since_epoch(&self) -> i64 {
        let month = i64::from(self.month);
        let year = i64::from(self.year) - i64::from(month <= 2);
        let era = year.div_euclid(400);
        let year_of_era = year - era * 400;
        let shifted_month = (month + 9) % 12;
        let day_of_year = (153 * shifted_month + 2) / 5 + i64::from(self.day) - 1;
        let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
        era * 146_097 + day_of_era - 719_468
    }

    /// Weekday index, Monday = 0.
    pub fn weekday(&self) -> usize {
        // 1970-01-01 was a Thursday (index 3).
        (self.days_since_epoch() + 3).rem_euclid(7) as usize
    }

    /// RFC 2822 timestamp at midnight UTC, as required by RSS `pubDate`.
    pub fn rfc2822(&self) -> String {
        format!(
            "{}, {:02} {} {:04} 00:00:00 +0000",
            WEEKDAYS[self.weekday()],
            self.day,
            MONTHS[usize::from(self.month) - 1],
            self.year
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_dates() {
        let date = Date::parse("2024-02-29").unwrap();
        assert_eq!(
            date,
            Date {
                year: 2024,
                month: 2,
                day: 29
            }
        );
        assert_eq!(date.iso(), "2024-02-29");
        assert_eq!(date.finnish(), "29.2.2024");
    }

    #[test]
    fn rejects_invalid_dates() {
        for value in [
            "2023-02-29",
            "1900-02-29",
            "2024-13-01",
            "2024-00-10",
            "2024-04-31",
            "2024-04-00",
            "2024/04/01",
            "24-04-01",
            "abcd-ef-gh",
            "2024-1a-01",
            "2024-01-1a",
        ] {
            assert!(Date::parse(value).is_err(), "{value} should be rejected");
        }
        assert!(Date::parse("2000-02-29").is_ok());
        assert_eq!(
            Date::parse("x").unwrap_err().to_string(),
            "invalid date 'x', expected YYYY-MM-DD"
        );
    }

    #[test]
    fn computes_weekdays() {
        assert_eq!(Date::parse("1970-01-01").unwrap().weekday(), 3);
        assert_eq!(Date::parse("2026-11-14").unwrap().weekday(), 5);
        assert_eq!(Date::parse("2000-01-01").unwrap().weekday(), 5);
        assert_eq!(Date::parse("1969-12-29").unwrap().weekday(), 0);
    }

    #[test]
    fn formats_rfc2822() {
        assert_eq!(
            Date::parse("2026-01-15").unwrap().rfc2822(),
            "Thu, 15 Jan 2026 00:00:00 +0000"
        );
        assert_eq!(
            Date::parse("2025-12-07").unwrap().rfc2822(),
            "Sun, 07 Dec 2025 00:00:00 +0000"
        );
    }

    #[test]
    fn orders_chronologically() {
        let older = Date::parse("2025-12-31").unwrap();
        let newer = Date::parse("2026-01-01").unwrap();
        assert!(older < newer);
    }
}
