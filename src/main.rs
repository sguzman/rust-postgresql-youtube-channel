extern crate rand;

mod lib;



fn main() {
    let chans = lib::channels("root", "", "localhost", 5432, "youtube");
    println!("Retrieved {} channels", chans.len());
    let chans_priors = lib::prior_adjust(chans);

    println!("{}", chans_priors.len());
}