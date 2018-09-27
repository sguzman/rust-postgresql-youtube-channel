extern crate postgres;

use postgres::{Connection, TlsMode};

pub struct Channel {
    pub channel_serial: String,
    pub id: i64
}

pub fn connect(user: &str, pass: &str, host: &str, port: u16, db: &str) -> postgres::Connection {
    let postgres_url = format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, db);

    let conn = Connection::connect(postgres_url, TlsMode::None)
        .unwrap();

    return conn;
}