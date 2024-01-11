mod ping_result;
mod database;
mod config;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::fs::read_to_string;
use std::thread::sleep;

use crate::database::Database;
use crate::ping_result::PingResult;
use crate::config::Config;


fn parse_ip(input: &str) -> Option<IpAddr> {
    // Try parsing as IPv4
    if let Ok(v4_addr) = input.parse::<Ipv4Addr>() {
        return Some(IpAddr::V4(v4_addr));
    }

    // Try parsing as IPv6
    if let Ok(v6_addr) = input.parse::<Ipv6Addr>() {
        return Some(IpAddr::V6(v6_addr));
    }

    // Return None if both parsing attempts fail
    None
}


fn run(cfg: &Config) {
    let db = Database::new(cfg.app.database_path.as_str());
    db.create_tables();
    
    loop {
        for (_i, line) in read_to_string(cfg.app.whitelist_path.as_str()).unwrap().lines().enumerate() {
    
            match parse_ip(line){
                Some(ip) => {
                    let ping_result = PingResult::ping(ip);
                    db.insert_result(&ping_result);
                },
                None => println!("Error parsing line: {}", line)
            }
        }
        sleep(std::time::Duration::from_secs(cfg.app.ping_interval));
    }
}


fn main() {

    let config = Config::load("./config/config.yaml");

    println!("Application started with config: ");
    println!("Config: {:?}", config);

    run(&config);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ip() {
        let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        assert_eq!(parse_ip("127.0.0.1").unwrap(), localhost);

        let ipv6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
        assert_eq!(parse_ip("::1").unwrap(), ipv6);

        let ipv62 = IpAddr::V6(Ipv6Addr::new(1111, 2222, 3333, 4444, 5555, 6666, 7777, 8888));
        println!("{}", ipv62.to_string());
        assert_eq!(parse_ip("457:8ae:d05:115c:15b3:1a0a:1e61:22b8").unwrap(), ipv62);

        let invalid = "dfkgjhsd";
        assert!(parse_ip(invalid).is_none());
    }
}