extern crate reqwest;

mod chan;
mod http;

fn main() {
    let chans_priors = chan::main();
    println!("{}", chans_priors.len());

    for s in chans_priors {
        let serial = s.channel_serial;

        let client = reqwest::Client::new();
        let info = http::get(client, serial);
        println!("{}, {}", info.0, info.1);
    }
}