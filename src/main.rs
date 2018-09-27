mod lib;

fn main() {
    let chans = lib::channels("root", "", "localhost", 5432, "youtube");

    println!("Retrieved {} channels", chans.len());

}