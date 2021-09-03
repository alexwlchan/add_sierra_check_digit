mod sierra;

use docopt::Docopt;
use serde::Deserialize;
use substring::Substring;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

const USAGE: &'static str = "
Work with Sierra record numbers.

Usage:
    sierra_numbers add-check-digit <number>
    sierra_numbers validate <number>
    sierra_numbers (-h | --help)
    sierra_numbers --version

Options:
    -h --help           Show this screen.
    --version           Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_number: String,
    cmd_add_check_digit: bool,
    cmd_validate: bool,
    flag_help: bool,
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("{}.rs v{}", NAME, VERSION);
    }

    if args.cmd_add_check_digit {
        let number = args.arg_number;

        if number.len() == 7 && sierra::is_numeric(&number) {
            println!("{}{}", number, sierra::get_check_digit(&number));
        } else if number.len() == 8 && sierra::is_numeric(&number.substring(1, 8)) {
            let record_type_prefix = number.substring(0, 1);
            let record_number = number.substring(1, 8);
            println!("{}{}{}", record_type_prefix, record_number, sierra::get_check_digit(&record_number));
        } else {
            eprintln!("Not a valid Sierra number: {}", number);
            std::process::exit(1);
        }
    } else if args.cmd_validate {
        if sierra::validate(&args.arg_number) {
            std::process::exit(0);
        } else {
            std::process::exit(1);
        }
    }
}
