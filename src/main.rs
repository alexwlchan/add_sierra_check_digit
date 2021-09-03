mod sierra;

use std::env;

use substring::Substring;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let number = &args[1];

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
    } else {
        eprintln!("Usage: {} <NUMBER>", &args[0]);
        std::process::exit(1);
    }
}
