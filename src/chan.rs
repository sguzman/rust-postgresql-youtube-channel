extern crate postgres;
extern crate rand;

const CHAN_LEN: usize = 10000;

const SQL_USER: &str = "root";
const SQL_PASS: &str = "";
const SQL_HOST: &str = "localhost";
const SQL_PORT: u16 = 5432;
const SQL_DB: &str = "youtube";

#[derive(Clone)]
pub struct Channel {
    pub id: i64,
    pub channel_serial: String
}

fn connect() -> postgres::Connection {
    let postgres_url: String = format!("postgres://{}:{}@{}:{}/{}", SQL_USER, SQL_PASS, SQL_HOST, SQL_PORT, SQL_DB);
    let none = postgres::TlsMode::None;

    let conn = postgres::Connection::connect(postgres_url, none)
        .unwrap();

    return conn;
}

fn channels() -> Vec<Channel> {
    let query_str: String = format!("SELECT chan_serial, id FROM youtube.channels.chans ORDER BY subs DESC LIMIT {}", CHAN_LEN);

    let conn = connect();
    let query_results = conn.query(query_str.as_ref(), &[]).unwrap();

    let mut vec = Vec::new();
    for i in 0..query_results.len() {
        let row = query_results.get(i);
        let channel_serial: String = row.get(0);
        let id = row.get(1);

        let chan = Channel {
            channel_serial,
            id
        };
        vec.push(chan);
    }

    conn.finish().unwrap();
    vec
}

fn priority_weight(len: usize, idx: usize) -> usize {
    let weight = ((len / (1 + idx)) * (len / (1 + idx))) / len;
    return if weight == 0 {
        1
    } else {
        weight
    }
}

fn prior_adjust(chans: Vec<Channel>) -> Vec<Channel> {
    use rand::Rng;
    let mut priors = Vec::new();

    for i in 0..chans.len() {
        let prior_i = priority_weight(chans.len(), i);
        for _ in 0..prior_i {
            priors.push(chans[i].clone());
        }
    }

    let mut rng = rand::thread_rng();
    rng.shuffle(&mut priors);

    priors
}

pub fn main() -> Vec<Channel> {
    prior_adjust(channels())
}