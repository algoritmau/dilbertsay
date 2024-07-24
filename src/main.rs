extern crate structopt;

mod ascii_art;

use ascii_art::get_ascii_art;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "When did ignorance become a point of view?")]
    /// What's on Dilbert's head now?
    message: String,

    #[structopt(short = "c", long = "character", default_value = "dilbert")]
    /// dilbert, dogbert, tim, alice, catbert, or wally
    character: String,

    #[structopt(short = "f", long = "full")]
    /// Show character in full (only available for dilbert)
    full: bool
}

fn main() {
    let options = Options::from_args();
    let message = options.message;
    let mut ascii_art = get_ascii_art(String::from(options.character));

    if options.full {
        ascii_art = get_ascii_art(String::from("dilbert_full"));
    }

    println!("
    {message}
    {ascii_art}
    ");
}
