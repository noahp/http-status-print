extern crate clap;
extern crate http;

use clap::{App, Arg};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Noah P https://github.com/noahp")
        .about("Print information about HTTP status codes")
        .arg(
            Arg::with_name("CODE")
                .takes_value(true)
                .multiple(true)
                .required(true)
                .help("Input codes to return information on"),
        )
        .get_matches();

    let codes: Vec<&str> = matches.values_of("CODE").unwrap().collect();

    for code in codes {
        let intcode: u16 = match code.parse() {
            Ok(n) => n,
            Err(t) => {
                println!("bad error code: {}", t);
                continue;
            },
        };

        let status = match http::status::StatusCode::from_u16(intcode) {
            Ok(n) => n,
            Err(t) => {
                println!("bad error code {}: {}", intcode, t);
                continue;},
        };

        println!("{} : https://httpstatuses.com/{}", status, intcode);
    }
}
