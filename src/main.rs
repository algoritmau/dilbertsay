extern crate structopt;

mod ascii_art;

use ascii_art::get_ascii_art;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    message: String // [1]
}

fn main() {
    let options = Options::from_args();  // [2]
    //let message = std::env::args().nth(1)
    //    .expect("Expected a message but received none.\n\tUsage: \"dilbertsay <message>\"");
    let message = options.message;
    let ascii_art = get_ascii_art(String::from("dilbert"));

    println!("
    {message}
    {ascii_art}
    ");
}
