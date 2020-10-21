mod url;
extern crate clap;

use clap::{ App, Arg };

fn main() {
    let matches = App::new("Simple CLI URL Shortener")
    .version("1.0")
    .author("Matan Budimir")
    .about("Shortens given URLs.")
    .arg(Arg::with_name("url")
        .short("u")
        .long("url")
        .help("Sets the url")
        .allow_hyphen_values(true)
        .required(true)
        )
    .get_matches();

    println!("{}", url::get_short_url(matches.value_of("url").unwrap()));
}