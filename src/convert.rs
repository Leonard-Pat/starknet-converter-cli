use hex::FromHex;
use starknet::core::types::FieldElement;
use starknet::core::utils::cairo_short_string_to_felt;

#[derive(Debug)]
pub enum InputType {
    Hex,
    Felt,
    String,
}

pub struct ReturnType {
    pub hex: Option<String>,
    pub felt: Option<String>,
    pub string: Option<String>,
}

pub trait ParseTypeTrait {
    fn find_and_validate_type(&self) -> Result<InputType, String>;
    fn convert_type(
        &self,
        input_type: InputType,
        felt: bool,
        hex: bool,
        string: bool,
    ) -> ReturnType;
}

impl ParseTypeTrait for String {
    fn find_and_validate_type(&self) -> Result<InputType, String> {
        if self.starts_with("0x") {
            if self[2..].chars().all(|c| c.is_ascii_hexdigit()) {
                return Ok(InputType::Hex);
            } else {
                return Err(format!("Invalid hexadecimal string: {}", self));
            }
        } else if self.chars().any(|c| c.is_numeric()) {
            if FieldElement::from_dec_str(self).is_err() {
                return Err(format!("Invalid felt (felt too long): {}", self));
            } else {
                return Ok(InputType::Felt);
            }
        } else {
            if self.chars().count() <= 31 {
                return Ok(InputType::String);
            } else {
                return Err(format!(
                    "Invalid short string {}, must be less than 32 characters",
                    self
                ));
            }
        }
    }

    fn convert_type(
        &self,
        input_type: InputType,
        mut felt: bool,
        mut hex: bool,
        mut string: bool,
    ) -> ReturnType {
        let mut response: ReturnType = ReturnType {
            hex: None,
            felt: None,
            string: None,
        };
        match input_type {
            InputType::Hex => {
                if !hex && !felt && !string {
                    felt = true;
                    hex = true;
                    string = true;
                }
                if hex {
                    response.hex = Some(self.to_string());
                }
                if felt {
                    response.felt = Some(FieldElement::from_hex_be(&self).unwrap().to_string());
                }
                if string {
                    let s = String::from(self);
                    let buffer = <[u8; 12]>::from_hex(&s[2..]).ok();
                    match buffer {
                        Some(buffer) => {
                            let string = core::str::from_utf8(&buffer).unwrap_or("");
                            response.string = Some(string.to_string());
                        }
                        None => {
                            response.string = None;
                        }
                    }
                }

                return response;
            }
            InputType::Felt => {
                if !hex && !felt && !string {
                    felt = true;
                    hex = true;
                    string = true;
                }
                if hex {
                    response.hex = Some("hello".to_string())
                }
                if felt {
                    response.felt = Some(FieldElement::from_dec_str(self).unwrap().to_string());
                }
                if string {
                    response.string = Some("hello".to_string())
                }
                return response;
            }
            InputType::String => {
                if !hex && !felt && !string {
                    felt = true;
                    hex = true;
                    string = true;
                }
                if hex {
                    response.hex = Some(hex::encode(self))
                }
                if felt {
                    response.felt = Some(cairo_short_string_to_felt(self).unwrap().to_string());
                }
                if string {
                    response.string = Some(self.to_string())
                }
                return response;
            }
        }
    }
}
