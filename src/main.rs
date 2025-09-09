use clap::Parser;
use colored::*;

#[derive(Debug, Parser)]
#[command(color = clap::ColorChoice::Always)]
struct Opt {
    code: Vec<u16>,
}

fn main() {
    human_panic::setup_panic!();

    let opt = Opt::parse();
    for code in opt.code {
        // Similar for codes that aren't present in the http::status list
        let status = match http::status::StatusCode::from_u16(code) {
            Ok(n) => n,
            Err(t) => {
                println!("bad error code \"{}\": {}", code, t);
                continue;
            }
        };

        // Only print a link to the expanded descriptions for known codes
        match status.canonical_reason() {
            Some(reason) => {
                let colored_code = match code {
                    100..=199 => code.to_string().blue(),    // Informational
                    200..=299 => code.to_string().green(),   // Success
                    300..=399 => code.to_string().yellow(),  // Redirection
                    400..=499 => code.to_string().red(),     // Client Error
                    500..=599 => code.to_string().magenta(), // Server Error
                    _ => code.to_string().white(),           // Unknown
                };
                println!(
                    "{} - {} : https://httpstatuses.com/{}",
                    colored_code, reason, code
                )
            }
            None => println!("{} - {}", code, status),
        };
    }
}
