use starknet_core::types::Felt;
use starknet_core::utils::cairo_short_string_to_felt;

#[derive(Debug, Clone, Copy)]
pub enum InputType {
    Hex,
    Felt,
    String,
}

#[derive(Debug)]
pub struct ConversionResult {
    pub hex: Option<String>,
    pub felt: Option<String>,
    pub string: Option<String>,
}

pub fn identify_input_type(input: &str) -> Result<InputType, String> {
    if input.starts_with("0x") && input[2..].chars().all(|c| c.is_ascii_hexdigit()) {
        Ok(InputType::Hex)
    } else if input.chars().all(|c| c.is_ascii_digit()) {
        Ok(InputType::Felt)
    } else if input.chars().any(|c| !c.is_ascii_hexdigit()) {
        // If it contains any non-hex characters, consider it a string
        Ok(InputType::String)
    } else {
        Err("Invalid input format".to_string())
    }
}

pub fn convert(input: &str, input_type: InputType) -> Result<ConversionResult, String> {
    match input_type {
        InputType::Hex => convert_from_hex(input),
        InputType::Felt => convert_from_felt(input),
        InputType::String => convert_from_string(input),
    }
}

fn convert_from_hex(input: &str) -> Result<ConversionResult, String> {
    // Remove '0x' prefix if present
    let hex_str = input.strip_prefix("0x").unwrap_or(input);

    // Pad with a leading zero if the length is odd
    let padded_hex = if hex_str.len() % 2 != 0 {
        format!("0{}", hex_str)
    } else {
        hex_str.to_string()
    };

    let felt =
        Felt::from_hex(&format!("0x{}", padded_hex)).map_err(|e| format!("Invalid hex: {}", e))?;

    let bytes = hex::decode(&padded_hex).map_err(|e| format!("Invalid hex: {}", e))?;

    let string = String::from_utf8(bytes).ok().filter(|s| s.len() <= 31);

    Ok(ConversionResult {
        hex: Some(input.to_string()),
        felt: Some(felt.to_string()),
        string,
    })
}

fn convert_from_felt(input: &str) -> Result<ConversionResult, String> {
    let felt = Felt::from_dec_str(input).map_err(|e| e.to_string())?;
    let hex = format!("0x{}", hex::encode(felt.to_bytes_be()));
    let string = String::from_utf8(felt.to_bytes_be().to_vec())
        .ok()
        .filter(|s| s.len() <= 31);

    Ok(ConversionResult {
        hex: Some(hex),
        felt: Some(input.to_string()),
        string,
    })
}

fn convert_from_string(input: &str) -> Result<ConversionResult, String> {
    if input.len() > 31 {
        return Err("String too long (max 31 characters)".to_string());
    }

    let felt = cairo_short_string_to_felt(input).unwrap().to_string();
    let hex = format!("0x{}", hex::encode(input.as_bytes()));

    Ok(ConversionResult {
        hex: Some(hex),
        felt: Some(felt),
        string: Some(input.to_string()),
    })
}
