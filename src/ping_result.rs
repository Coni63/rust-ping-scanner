use std::fmt;
use std::net::IpAddr;
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};
use ping::rawsock::ping;

pub struct PingResult {
    pub datetime: DateTime<Utc>,
    pub success_call: i32,
    pub average_response_ms: i32,
    pub host: IpAddr,
}

impl PingResult {
    pub fn new(host: IpAddr, success_call: i32, average_response_ms: i32) -> PingResult {
        PingResult {
            datetime: Utc::now(),
            success_call,
            average_response_ms,
            host: host,
        }
    }

    pub fn ping(host: IpAddr) -> PingResult {
        let timeout_limit = Duration::new(1, 0); // 1 second

        let start_time = Instant::now();
        let mut success_call = 0;
        let mut elapsed_time = 0;

        for _ in 0..32 {
            match ping(host, Some(timeout_limit), None, None, None, None) {
                Ok(_) => {
                    success_call += 1;
                    elapsed_time = start_time.elapsed().as_millis();
                },
                Err(err) => {
                    println!("Error: {}", err);
                    break;
                },
            }
        }

        PingResult::new(host, success_call, (elapsed_time as i32) / success_call)
    }
}

impl fmt::Display for PingResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PingResult: IpAddr: {}, success_call: {}, average_response_ms: {}",
            &self.host, &self.success_call, &self.average_response_ms
        )
    }
}


#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr};
    use super::*;

    #[test]
    fn test_ping_new() {
        let host = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let ping_result = PingResult::new(host, 1, 2);
        assert_eq!(ping_result.host, host);
        assert_eq!(ping_result.success_call, 1);
        assert_eq!(ping_result.average_response_ms, 2);
    }

    #[test]
    fn test_ping_ping() {
        let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let ping_result = PingResult::ping(localhost);
        assert_eq!(ping_result.success_call, 32);
        assert!(ping_result.average_response_ms < 100);
    }

    #[test]
    fn test_ping_display() {
        let host = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let ping_result = PingResult::new(host, 1, 2);
        assert_eq!(
            ping_result.to_string(),
            format!(
                "PingResult: IpAddr: {}, success_call: {}, average_response_ms: {}",
                 ping_result.host, ping_result.success_call, ping_result.average_response_ms
            )
        );
    }

    #[test]
    fn test_parse_ip() {
        let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        println!("{}", localhost.to_string());

        let ip = IpAddr::V4("8.0.8.0".parse::<Ipv4Addr>().unwrap());
        println!("{:?}", ip);
        
        let ip = IpAddr::V6("2a00:1450:4007:80d::200e".parse::<Ipv6Addr>().unwrap());
        println!("{:?}", ip);
    }
}