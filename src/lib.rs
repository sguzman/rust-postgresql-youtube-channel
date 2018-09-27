extern crate postgres;

pub struct Channel {
    pub channel_serial: String,
    pub id: i64
}

fn connect(user: &str, pass: &str, host: &str, port: u16, db: &str) -> postgres::Connection {
    let postgres_url = format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, db);

    let conn = postgres::Connection::connect(postgres_url, postgres::TlsMode::None)
        .unwrap();

    return conn;
}

pub fn channels(user: &str, pass: &str, host: &str, port: u16, db: &str) -> Vec<Channel> {
    let conn = connect(user, pass, host, port, db);
    let query_results = conn.query("SELECT channel_id, id FROM youtube.channels.channels", &[]).unwrap();

    let mut vec = Vec::new();
    for row in &query_results {
        let chan = Channel {
            channel_serial: row.get(0),
            id: row.get(1)
        };
        vec.push(chan);
    }

    conn.finish().unwrap();
    return vec;
}