//! Durations and timestamps used by episodes and the audio player.

use std::fmt;

/// Why a timestamp string was rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimestampError(pub String);

impl fmt::Display for TimestampError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid timestamp '{}', expected mm:ss or h:mm:ss",
            self.0
        )
    }
}

impl std::error::Error for TimestampError {}

/// Parses `mm:ss` or `h:mm:ss` into seconds.
pub fn parse_timestamp(value: &str) -> Result<u32, TimestampError> {
    let err = || TimestampError(value.to_string());
    let parts: Vec<&str> = value.trim().split(':').collect();
    if !(2..=3).contains(&parts.len()) {
        return Err(err());
    }
    let numbers = parts
        .iter()
        .map(|part| {
            if part.is_empty() || !part.bytes().all(|b| b.is_ascii_digit()) {
                return Err(err());
            }
            part.parse::<u32>().map_err(|_| err())
        })
        .collect::<Result<Vec<u32>, _>>()?;
    // Minutes and seconds after the leading unit must stay below 60.
    if numbers[1..].iter().any(|n| *n >= 60) {
        return Err(err());
    }
    Ok(numbers.iter().fold(0, |total, n| total * 60 + n))
}

/// Player clock: `4:05`, `1:02:03`.
pub fn clock(total_seconds: u32) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

/// Human readable Finnish length: `45 min`, `1 h 5 min`.
pub fn human_duration(total_seconds: u32) -> String {
    let total_minutes = (total_seconds + 30) / 60;
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;
    match (hours, minutes) {
        (0, m) => format!("{} min", m.max(1)),
        (h, 0) => format!("{h} h"),
        (h, m) => format!("{h} h {m} min"),
    }
}

/// ISO 8601 duration for JSON-LD: `PT45M12S`.
pub fn iso_duration(total_seconds: u32) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let mut out = String::from("PT");
    if hours > 0 {
        out.push_str(&format!("{hours}H"));
    }
    if minutes > 0 {
        out.push_str(&format!("{minutes}M"));
    }
    if seconds > 0 || total_seconds == 0 {
        out.push_str(&format!("{seconds}S"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_timestamps() {
        assert_eq!(parse_timestamp("12:34"), Ok(754));
        assert_eq!(parse_timestamp("0:05"), Ok(5));
        assert_eq!(parse_timestamp("1:02:03"), Ok(3723));
        assert_eq!(parse_timestamp(" 75:00 "), Ok(4500));
    }

    #[test]
    fn rejects_bad_timestamps() {
        for value in [
            "",
            "12",
            "1:2:3:4",
            "1:60",
            "1:60:00",
            "a:10",
            ":10",
            "1:-1",
            "99999999999:00",
        ] {
            assert!(parse_timestamp(value).is_err(), "{value} should fail");
        }
        assert_eq!(
            parse_timestamp("x").unwrap_err().to_string(),
            "invalid timestamp 'x', expected mm:ss or h:mm:ss"
        );
    }

    #[test]
    fn formats_clock() {
        assert_eq!(clock(0), "0:00");
        assert_eq!(clock(245), "4:05");
        assert_eq!(clock(3723), "1:02:03");
    }

    #[test]
    fn formats_human_duration() {
        assert_eq!(human_duration(10), "1 min");
        assert_eq!(human_duration(2712), "45 min");
        assert_eq!(human_duration(3600), "1 h");
        assert_eq!(human_duration(3900), "1 h 5 min");
    }

    #[test]
    fn formats_iso_duration() {
        assert_eq!(iso_duration(0), "PT0S");
        assert_eq!(iso_duration(2712), "PT45M12S");
        assert_eq!(iso_duration(3600), "PT1H");
        assert_eq!(iso_duration(3723), "PT1H2M3S");
    }
}
