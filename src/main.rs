extern crate reqwest;
extern crate rand;
extern crate postgres;
extern crate serde_json;
extern crate scraper;
extern crate influent;

mod chan;
mod http;

use rand::distributions::Distribution;
use influent::client::Client;
use influent::client::Precision;
use std::thread;
use std::sync::mpsc::sync_channel;
use rayon::prelude::*;

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

fn loop_body(distro: &rand::distributions::WeightedChoice<chan::Channel>, tx: std::sync::mpsc::SyncSender<(String, u64)>) {
    let mut rng = rand::thread_rng();
    let c = distro.sample(&mut rng);

    let resp: Option<(String, u64)> = http::get(c.channel_serial);
    match resp {
        Some(data) => tx.send(data).unwrap(),
        None => {}
    }
}

pub fn insert_job() {
    let chans = chan::channels();

    let mut items: Vec<rand::distributions::Weighted<chan::Channel>> = Vec::new();
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
        let credentials = influent::client::Credentials {
            username: USER,
            password: PASS,
            database: DB
        };

        let url = format!("http://{}:{}", HOST, PORT);
        let hosts = vec![url.as_ref()];

        let client = influent::create_client(credentials, hosts);
        loop {
            let (title, subs) = rx.recv().unwrap();
            println!("{} {}", title, subs);
            let mut measurement = influent::measurement::Measurement::new("Channels");
            measurement.add_tag("name", title);
            measurement.add_field("subscriberCount", influent::measurement::Value::Integer(subs as i64));

            client.write_one(measurement, None);
        }
    });

    loop {
        (0..(CORES * 10000))
            .into_par_iter()
            .for_each(|_| {
                loop_body(&distro, tx.clone())
            });
    }
}

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(CORES).build_global().unwrap();

    insert_job();
}