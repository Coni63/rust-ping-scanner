use std::net::Ipv4Addr;

use rusqlite::Connection;

use crate::ping_result::PingResult;

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Database {
        let conn = Connection::open(path).unwrap();
        Database { conn }
    }

    pub fn create_tables(&self) {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS results (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                datetime TEXT NOT NULL,
                host TEXT NOT NULL,
                success_call INTEGER DEFAULT 0,
                average_response_ms INTEGER DEFAULT 0
                )", [],
            )
            .expect("Error creating results table");
    }

    pub fn insert_result(&self, ping_result: &PingResult) {
        self.conn
            .execute(
                "INSERT INTO results (datetime, host, success_call, average_response_ms) VALUES (?, ?, ?, ?)",
                (&ping_result.datetime.to_rfc3339(), &ping_result.host.to_string(), &ping_result.success_call, &ping_result.average_response_ms),
            )
            .expect("Failed to insert record");
    }

    #[allow(dead_code)]
    pub fn get_results(&self, limit: u32) -> Vec<PingResult> {
        let mut records: Vec<PingResult> = self.conn
            .prepare("SELECT datetime, host, success_call, average_response_ms FROM results ORDER BY id DESC LIMIT ?")
            .unwrap()
            .query_map([limit], |row| {
                Ok(PingResult {
                    datetime: row.get(0)?,
                    host: {
                        let host_str: String = row.get(1).unwrap();
                        host_str.parse::<Ipv4Addr>().unwrap().into()
                    },
                    success_call: row.get(2)?,
                    average_response_ms: row.get(3)?,
                })
            })
            .unwrap()
            .filter_map(|result| result.ok())
            .collect();
        
        records.reverse();

        records
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::SystemTime;
    use std::net::{Ipv4Addr, IpAddr};

    #[test]
    fn test_database_create_tables() {
        remove_file("./test.sqlite");
        let db = Database::new("./test.sqlite");
        db.create_tables();

        let query = "SELECT COUNT(name) FROM sqlite_master WHERE type='table' AND name='results'";

        let count: i32 = db.conn
        .query_row(
            query, [], |row| row.get(0),
        )
        .unwrap_or(0);

        db.conn.close().unwrap();

        assert_eq!(count, 1);

        remove_file("./test.sqlite");
    }

    #[test]
    fn test_database_insert_and_get_results() {
        remove_file("./test.sqlite");

        let db = Database::new("./test2.sqlite");
        db.create_tables();

        let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

        for i in 0..5 {
            let ping_result = PingResult::new(localhost, i+2, i*2+3);
            db.insert_result(&ping_result);
        }

        let ping_results = db.get_results(3);
        
        db.conn.close().unwrap();

        assert_eq!(ping_results.len(), 3);
        let ping_result = &ping_results[0];

        let timedelta = SystemTime::now().duration_since(ping_result.datetime.into()).unwrap().as_millis();

        assert!(timedelta < 10000);
        assert_eq!(ping_result.host, localhost);
        assert_eq!(ping_result.success_call, 4);
        assert_eq!(ping_result.average_response_ms, 7);

        remove_file("./test2.sqlite");
    }

    fn remove_file(file_path: &str) {
        match fs::remove_file(file_path) {
            Ok(_) => {},
            Err(e) => { 
                if e.kind() != std::io::ErrorKind::NotFound {
                    panic!("Failed to remove file {}: {}", file_path, e);
                }
            }
        }
    }
}