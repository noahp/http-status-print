extern crate clap;
extern crate http;

use clap::{App, Arg};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Noah Pendleton / https://github.com/noahp")
        .about("Print information about HTTP status codes")
        .arg(
            Arg::with_name("CODE")
                .takes_value(true)
                .multiple(true)
                .required(true)
                .help("Numerical HTTP codes to return information on"),
        )
        .get_matches();

    let codes: Vec<&str> = matches.values_of("CODE").unwrap().collect();

    for code in codes {
        // Handle u16 parsing errors with a friendly response
        let intcode: u16 = match code.parse() {
            Ok(n) => n,
            Err(t) => {
                println!("bad error code \"{}\": {}", code, t);
                continue;
            }
        };

        // Similar for codes that aren't present in the http::status list
        let status = match http::status::StatusCode::from_u16(intcode) {
            Ok(n) => n,
            Err(t) => {
                println!("bad error code \"{}\": {}", intcode, t);
                continue;
            }
        };

        // Only print a link to the expanded descriptions for known codes
        match status.canonical_reason() {
            Some(reason) => println!("{} : https://httpstatuses.com/{}", reason, intcode),
            None => println!("{}", status),
        };
    }
}

