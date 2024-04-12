mod convert;
use clap::Parser;
use convert::ParseTypeTrait;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// felt, string or hex to convert
    input: String,

    #[arg(short, long, action)]
    string: bool,

    #[arg(short, long, action)]
    felt: bool,

    #[arg(long, action)]
    hex: bool,
}

fn main() {
    let args = Args::parse();

    let input = args.input;
    match input.find_and_validate_type() {
        Ok(input_type) => {
            let result = input.convert_type(input_type, args.felt, args.hex, args.string);

            println!("Hex: {:?}", result.hex);
            println!("Felt: {:?}", result.felt);
            println!("String: {:?}", result.string);
        }
        Err(error_message) => {
            eprintln!("{}", error_message);
        }
    }
}
