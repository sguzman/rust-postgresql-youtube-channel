extern crate postgres;
extern crate rand;

const CHAR_LEN: usize = 24;
const PRIOR_LEN: usize = 26203;
const CHAN_LEN: usize = 10000;

#[derive(Copy, Clone)]
pub struct Channel {
    pub channel_serial: [char; CHAR_LEN],
    pub id: i64
}

const CHAN_NIL: Channel = Channel {
  channel_serial: ['\0'; CHAR_LEN],
  id: 0
};

fn connect(user: &str, pass: &str, host: &str, port: u16, db: &str) -> postgres::Connection {
    let postgres_url: String = format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, db);
    let none = postgres::TlsMode::None;

    let conn = postgres::Connection::connect(postgres_url, none)
        .unwrap();

    return conn;
}

pub fn channels(user: &str, pass: &str, host: &str, port: u16, db: &str) -> [Channel; CHAN_LEN] {
    let query_str: String = format!("SELECT channel_id, id FROM youtube.channels.channels LIMIT {}", CHAN_LEN);

    let conn = connect(user, pass, host, port, db);
    let query_results = conn.query(query_str.as_ref(), &[]).unwrap();

    let mut vec: [Channel; CHAN_LEN] = [CHAN_NIL; CHAN_LEN];
    for i in 1..query_results.len() {
        let row = query_results.get(i);
        let serial: String = row.get(0);
        serial.chars();

        let chan = Channel {
            channel_serial: {
                let mut chars = ['\0'; CHAR_LEN];
                for i in 0..(serial.len() - 1) {
                    chars[i] = serial.as_bytes()[i] as char;
                }

                chars
            },
            id: row.get(1)
        };
        vec[i] = chan;
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

pub fn prior_adjust(chans: [Channel; CHAN_LEN]) -> [Channel; PRIOR_LEN] {
    let mut priors = [CHAN_NIL; PRIOR_LEN];

    let mut idx = 0;
    for i in 0..(chans.len() - 1) {
        let prior_i = priority_weight(chans.len(), i);
        for _ in 0..prior_i {
            priors[idx] = chans[i];
            idx += 1;
        }
    }

    priors
}