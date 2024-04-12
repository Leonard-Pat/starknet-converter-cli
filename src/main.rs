use clap::Parser;
use core::str;
use starknet::core::types::FieldElement;
use starknet::core::utils::cairo_short_string_to_felt;
use hex::FromHex;

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

#[derive(Debug)]
enum InputType {
    Hex,
    Felt,
    String,
}

struct ReturnType {
    hex: Option<String>,
    felt: Option<String>,
    string: Option<String>,
}

trait ParseType {
    fn find_and_validate_type(&self) -> Result<InputType, String>;
    fn convert_type(&self, input_type: InputType, felt: bool, hex: bool, string: bool) -> Result<ReturnType, String>;
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
        else if self.chars().any(|c| c.is_numeric()) {
            if FieldElement::from_dec_str(self).is_err(){
                return Err(format!("Invalid felt (felt too long): {}", self));
            } else {
                return Ok(InputType::Felt);
            }
        }
        else {
            if self.chars().count() <= 31 {
                return Ok(InputType::String);
            } else {
                return Err(format!("Invalid short string {}, must be less than 32 characters", self));
            }
        }
    }

    fn convert_type(&self, input_type: InputType, mut felt: bool, mut hex: bool, mut string: bool)-> Result<ReturnType, String> {
        let mut response: ReturnType = ReturnType {hex: None, felt: None, string: None};
        match input_type {
            InputType::Hex => {
                if !hex && !felt && !string {
                    felt = true;
                    hex = true;
                    string = true;
                }
                if hex {response.hex = Some(self.to_string());}
                if felt {response.felt = Some(FieldElement::from_hex_be(&self).unwrap().to_string());}
                if string {
                    let s = String::from(self);
                    let buffer = <[u8; 12]>::from_hex(&s[2..]).ok();
                    match buffer {
                        Some(buffer) => {
                            let string = str::from_utf8(&buffer).unwrap_or("");
                            response.string = Some(string.to_string());
                        }
                        None => {response.string = None;}      
                    }
                }
                
                return Ok(response);
            }
            InputType::Felt => {
                if !hex && !felt && !string {
                    felt = true;
                    hex = true;
                    string = true;
                }
                if hex {response.hex = Some("hello".to_string())}
                if felt {response.felt = Some(FieldElement::from_dec_str(self).unwrap().to_string());}
                if string {response.string = Some("hello".to_string())}
                return Ok(response);
            }
            InputType::String => {
                if !hex && !felt && !string {
                    felt = true;
                    hex = true;
                    string = true;
                }
                if hex {response.hex = Some(hex::encode(self))}
                if felt {response.felt = Some(cairo_short_string_to_felt(self).unwrap().to_string());}
                if string {response.string = Some(self.to_string())}
                return Ok(response);  
            }
    
    }
}
}



fn main() {
    let args = Args::parse();

    let input = args.input;
    println!("Input: {}", input);
    match input.find_and_validate_type() {
        Ok(input_type) => {
            let result = input.convert_type(input_type, args.felt, args.hex, args.string);
            match result {
                Ok(result) => {
                    println!("Hex: {:?}", result.hex);
                    println!("Felt: {:?}", result.felt);
                    println!("String: {:?}", result.string);
                }
                Err(error_message) => {
                    eprintln!("{}", error_message);
                }
            }
                } 
        Err(error_message) => {
            eprintln!("{}", error_message);
        }
    }
}