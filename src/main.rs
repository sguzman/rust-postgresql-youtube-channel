extern crate reqwest;

mod chan;
mod http;

pub fn insert_job(chans: Vec<chan::Channel>) {
    use rayon::prelude::*;
    use std::thread;
    //use std::sync::mpsc::sync_channel;

    /*let (tx, rx) = sync_channel::<i64>(100);
    thread::spawn(move||{
        let metric = rx.recv().unwrap();
        println!("Received something {}", metric);
    });
*/
    chans
        .into_par_iter()
        .for_each(|c: chan::Channel| {
            let res = http::get(c.channel_serial);
            match res {
                Some((title, subs)) => println!("{} {}", title, subs),
                None => {}
            }
        });
}

fn main() {
    let chans_priors = chan::main();
    insert_job(chans_priors);
}