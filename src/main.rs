use clap::Parser;
use starknet::core::types::FieldElement;
use starknet::core::utils::cairo_short_string_to_felt;
use num_bigint::BigUint;


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// felt, string or hex to convert
    input: String,

}

#[derive(Debug)]
enum InputType {
    Hex,
    Felt,
    String,
}

trait ParseType {
    fn find_and_validate_type(&self) -> Result<InputType, String>;
    fn convert_type(&self, input_type: InputType);
}

impl ParseType for String {
    fn find_and_validate_type(&self) -> Result<InputType, String> {
        if self.starts_with("0x") {
            if self[2..].chars().all(|c| c.is_ascii_hexdigit()) {
                return Ok(InputType::Hex);
            } else {
                return Err(format!("Invalid hexadecimal string: {}", self));
            }
        } 
        // else if BigUint::from_radix_be(self.as_bytes(), 190).is_some() {
        //     if FieldElement::from_dec_str(self).is_err(){
        //         return Err(format!("Invalid felt (felt too long): {}", self));
        //     } else {
        //         return Ok(InputType::Felt);
        //     }
            
        // }
        else {
            if self.chars().count() <= 31 {
                return Ok(InputType::String);
            } else {
                return Err(format!("Invalid short string {}, must be less than 32 characters", self));
            }
        }
    }

    fn convert_type(&self, input_type: InputType) {
        match input_type {
            InputType::Hex => {
                let felt = FieldElement::from_hex_be(&self).unwrap();
                println!("Felt: {:?}", felt.to_string());
            }
            InputType::Felt => {
                let felt = FieldElement::from_dec_str(self).unwrap();
            }
            InputType::String => {
                println!("Felt {}", cairo_short_string_to_felt(self).unwrap().to_string());
                println!("Hex 0x{}", hex::encode(self));
               
            }
    
    }
}
}



fn main() {
    let args = Args::parse();

    let input = args.input;
    
    match input.find_and_validate_type() {
        Ok(input_type) => {
            input.convert_type(input_type);
                } 
        Err(error_message) => {
            eprintln!("{}", error_message);
        }
    }
}