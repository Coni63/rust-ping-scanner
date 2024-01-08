use rusqlite::Connection;

use crate::ping::Ping;

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
                retry_count INTEGER DEFAULT 0,
                average_response_ms INTEGER DEFAULT 0
                )", [],
            )
            .expect("Error creating results table");
    }

    pub fn insert_result(&self, ping: &Ping) {
        self.conn
            .execute(
                "INSERT INTO results (datetime, retry_count, average_response_ms) VALUES (?, ?, ?)",
                (&ping.datetime.to_rfc3339(), &ping.retry_count, &ping.average_response_ms),
            )
            .expect("Failed to insert record");
    }

    pub fn get_results(&self, limit: u32) -> Vec<Ping> {
        let mut records: Vec<Ping> = self.conn
            .prepare("SELECT datetime, retry_count, average_response_ms FROM results ORDER BY id DESC LIMIT ?")
            .unwrap()
            .query_map([limit], |row| {
                Ok(Ping {
                    datetime: row.get(0)?,
                    retry_count: row.get(1)?,
                    average_response_ms: row.get(2)?,
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
    use std::time::{SystemTime};

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

        for i in 0..5 {
            let ping = Ping::new(i+2, i*2+3);
            db.insert_result(&ping);
        }

        let pings = db.get_results(3);
        
        db.conn.close().unwrap();

        assert_eq!(pings.len(), 3);
        let ping = &pings[0];

        let timedelta = SystemTime::now().duration_since(ping.datetime.into()).unwrap().as_millis();

        assert!(timedelta < 10000);
        assert_eq!(ping.retry_count, 4);
        assert_eq!(ping.average_response_ms, 7);

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