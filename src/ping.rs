use std::fmt;
use chrono::{DateTime, Utc};


pub struct Ping {
    pub datetime: DateTime<Utc>,
    pub retry_count: i32,
    pub average_response_ms: i32,
}

impl Ping {
    pub fn new(retry_count: i32, average_response_ms: i32) -> Ping {
        Ping {
            datetime: Utc::now(),
            retry_count,
            average_response_ms,
        }
    }
}

impl fmt::Display for Ping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ping: datetime: {}, retry_count: {}, average_response_ms: {}",
            &self.datetime.to_rfc3339(), &self.retry_count, &self.average_response_ms
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping_new() {
        let ping = Ping::new(1, 2);
        assert_eq!(ping.retry_count, 1);
        assert_eq!(ping.average_response_ms, 2);
    }

    #[test]
    fn test_ping_display() {
        let ping = Ping::new(1, 2);
        assert_eq!(
            ping.to_string(),
            format!(
                "Ping: datetime: {}, retry_count: {}, average_response_ms: {}",
                ping.datetime, ping.retry_count, ping.average_response_ms
            )
        );
    }
}