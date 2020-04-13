use structopt::clap::AppSettings;
use structopt::StructOpt;

#[structopt(global_settings = &[AppSettings::ColoredHelp])]
#[derive(Debug, StructOpt)]
struct Opt {
    code: Vec<u16>,
}

fn main() {
    human_panic::setup_panic!();

    let opt = Opt::from_args();
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
            // TODO gimme some nice colorization here please!
            Some(reason) => println!(
                "{} - {} : https://httpstatuses.com/{}",
                code, reason, code
            ),
            None => println!("{} - {}", code, status),
        };
    }
}
