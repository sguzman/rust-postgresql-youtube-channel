extern crate reqwest;

mod chan;
mod http;

const CORES: usize = 4;

pub fn insert_job(chans: Vec<chan::Channel>) {
    use rayon::prelude::*;
    use std::thread;
    use std::sync::mpsc::sync_channel;

    let (tx, rx) = sync_channel::<(String, u64)>(100);
    thread::spawn(move||{
        loop {
            let (title, subs) = rx.recv().unwrap();
            println!("{} {}", title, subs);
        }
    });

    chans
        .into_par_iter()
        .for_each(|c: chan::Channel| {
            let resp: Option<(String, u64)> = http::get(c.channel_serial);
            match resp {
                Some(data) => tx.send(data).unwrap(),
                None => {}
            }
        });
}

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(CORES).build_global().unwrap();

    let chans: Vec<chan::Channel> = chan::main();
    insert_job(chans);
}