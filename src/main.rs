use clap::Parser;


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// felt, string or hex to convert
    input: String,

}

enum InputType{
    Felt,
    String,
    Hex
}

trait ParseType {
    fn find_type(&self) -> InputType;
    fn validate_type(&self, input_type: InputType) -> bool;
}

impl ParseType for String {
    fn find_type(&self) -> InputType {
        if self.starts_with("0x") {
            return InputType::Hex;
        } else if self.parse::<u64>().is_ok() {
            return InputType::Felt;
        } else {
            return InputType::String;
        }
    }

    fn validate_type(&self, input_type: InputType) -> bool {
        match input_type {
            InputType::Felt => {
                println!("Is a felt {}", args.input);
            }
            InputType::String => {
                println!("Is a string {}", args.input);
            }
            InputType::Hex => {
                println!("Is a hex {}", args.input);
            }
        }
    }

}

fn main() {
    let args = Args::parse();

    match args.input.find_type() {
        InputType::Felt => {
            println!("Is a felt {}", args.input);
        }
        InputType::String => {
            println!("Is a string {}", args.input);
        }
        InputType::Hex => {
            println!("Is a hex {}", args.input);
        }
    }
}