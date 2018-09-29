extern crate reqwest;

mod chan;
mod http;

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
            let resp = http::get(c.channel_serial);
            match resp {
                Some(data) => tx.send(data).unwrap(),
                None => {}
            }
        });
}

fn main() {
    let chans_priors = chan::main();
    insert_job(chans_priors);
}