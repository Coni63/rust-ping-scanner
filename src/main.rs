mod ping_result;
mod database;

use std::net::{IpAddr, Ipv4Addr};

use crate::database::Database;
use crate::ping_result::PingResult;

fn main() {
    let db = Database::new("./results.sqlite");
    db.create_tables();

    // let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let localhost = IpAddr::V4(Ipv4Addr::new(8,8,8,8));
    let ping_result = PingResult::ping(localhost);

    db.insert_result(&ping_result);
}