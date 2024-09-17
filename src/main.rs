mod convert;
use clap::Parser;
use convert::{convert, identify_input_type, ConversionResult};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The input value to convert
    #[clap(value_parser)]
    input: String,

    /// Display the hexadecimal representation
    #[clap(short = 'x', long, value_parser, default_value_t = false)]
    hex: bool,

    /// Display the felt representation
    #[clap(short, long, value_parser, default_value_t = false)]
    felt: bool,

    /// Display the short string representation
    #[clap(short, long, value_parser, default_value_t = false)]
    string: bool,
}

fn main() {
    let cli = Cli::parse();

    let input_type = match identify_input_type(&cli.input) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let result = match convert(&cli.input, input_type) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    print_result(&result, cli.hex, cli.felt, cli.string);
}

fn print_result(result: &ConversionResult, hex: bool, felt: bool, string: bool) {
    if !hex && !felt && !string {
        print_all(result);
    } else {
        if hex {
            println!("Hex: {}", result.hex.as_deref().unwrap_or("N/A"));
        }
        if felt {
            println!("Felt: {}", result.felt.as_deref().unwrap_or("N/A"));
        }
        if string {
            println!(
                "Short String: {}",
                result.short_string.as_deref().unwrap_or("N/A")
            );
        }
    }
}

fn print_all(result: &ConversionResult) {
    println!("Hex: {}", result.hex.as_deref().unwrap_or("N/A"));
    println!("Felt: {}", result.felt.as_deref().unwrap_or("N/A"));
    println!(
        "Short String: {}",
        result.short_string.as_deref().unwrap_or("N/A")
    );
}
