extern crate reqwest;
extern crate rand;
extern crate postgres;
extern crate serde_json;
extern crate scraper;
extern crate influent;

mod chan;
mod http;

use influent::create_client;
use influent::client::{Client, Credentials};
use influent::measurement::{Measurement, Value};
use influent::client::Precision;

const USER: &str = "admin";
const PASS: &str = "admin";
const HOST: &str = "localhost";
const PORT: u16 = 8086;
const DB: &str = "youtube";

const CORES: usize = 4;

fn priority_weight(len: u32, idx: u32) -> u32 {
    let weight = ((len / (1 + idx)) * (len / (1 + idx))) / len;

    if weight == 0 {
        1
    } else {
        weight
    }
}

pub fn insert_job() {
    use std::thread;
    use std::sync::mpsc::sync_channel;
    use rand::distributions::{Weighted, WeightedChoice, Distribution};

    let chans = chan::channels();

    let mut items: Vec<Weighted<chan::Channel>> = Vec::new();
    let len = chans.len();

    for i in 0..len {
        items.push(rand::distributions::Weighted {
            weight: priority_weight(len as u32, i as u32),
            item: chans[i].clone()
        });
    }

    let distro = rand::distributions::WeightedChoice::new(&mut items);

    let (tx, rx) = sync_channel::<(String, u64)>(100);
    thread::spawn(move||{
        let credentials = Credentials {
            username: USER,
            password: PASS,
            database: DB
        };

        let url = format!("http://{}:{}", HOST, PORT);
        let hosts = vec![url.as_ref()];

        let client = create_client(credentials, hosts);
        loop {
            let (title, subs) = rx.recv().unwrap();
            println!("{} {}", title, subs);
            let mut measurement = Measurement::new("Channels");
            measurement.add_tag("name", title);
            measurement.add_field("subscriberCount", Value::Integer(subs as i64));

            client.write_one(measurement, None);
        }
    });

    loop {
        let mut rng = rand::thread_rng();
        let c = distro.sample(&mut rng);

        let resp: Option<(String, u64)> = http::get(c.channel_serial);
        match resp {
            Some(data) => tx.send(data).unwrap(),
            None => {}
        }
    }
}

fn main() {
    insert_job();
}