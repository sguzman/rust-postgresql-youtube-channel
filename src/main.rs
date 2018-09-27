extern crate rand;

mod lib;

fn main() {
    let chans_priors = lib::channels::main();

    for i in 0..(chans_priors.len()) {
        println!("{}", chans_priors[i].id)
    }
}