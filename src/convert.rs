use starknet_core::types::Felt;
use starknet_core::utils::{cairo_short_string_to_felt, parse_cairo_short_string};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputType {
    Hex,
    Felt,
    ShortString,
}

#[derive(Debug)]
pub struct ConversionResult {
    pub hex: Option<String>,
    pub felt: Option<String>,
    pub short_string: Option<String>,
}

pub fn identify_input_type(input: &str) -> Result<InputType, String> {
    if input.starts_with("0x") && input[2..].chars().all(|c| c.is_ascii_hexdigit()) {
        Ok(InputType::Hex)
    } else if input.chars().all(|c| c.is_ascii_digit()) {
        Ok(InputType::Felt)
    } else if input.chars().any(|c| !c.is_ascii_hexdigit()) {
        Ok(InputType::ShortString)
    } else {
        Err("Invalid input format".to_string())
    }
}

pub fn convert(input: &str, input_type: InputType) -> Result<ConversionResult, String> {
    match input_type {
        InputType::Hex => convert_from_hex(input),
        InputType::Felt => convert_from_felt(input),
        InputType::ShortString => convert_from_string(input),
    }
}

fn convert_from_hex(input: &str) -> Result<ConversionResult, String> {
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
        short_string: string,
    })
}

fn convert_from_felt(input: &str) -> Result<ConversionResult, String> {
    let felt = Felt::from_dec_str(input).map_err(|e| e.to_string())?;
    let hex = format!("{}", felt.to_hex_string());
    let string = parse_cairo_short_string(&felt).unwrap();

    Ok(ConversionResult {
        hex: Some(hex),
        felt: Some(input.to_string()),
        short_string: Some(string),
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
        short_string: Some(input.to_string()),
    })
}

#[test]
fn test_convert_from_hex() {
    let result = convert_from_hex("0x48656c6c6f").unwrap();
    assert_eq!(result.hex, Some("0x48656c6c6f".to_string()));
    assert_eq!(result.felt, Some("310939249775".to_string()));
    assert_eq!(result.short_string, Some("Hello".to_string()));
}

#[test]
fn test_convert_from_felt() {
    let result = convert_from_felt("310939249775").unwrap();
    assert_eq!(result.hex, Some("0x48656c6c6f".to_string()));
    assert_eq!(result.felt, Some("310939249775".to_string()));
    assert_eq!(result.short_string, Some("Hello".to_string()));
}

#[test]
fn test_convert_from_string() {
    let result = convert_from_string("Hello").unwrap();
    assert_eq!(result.hex, Some("0x48656c6c6f".to_string()));
    assert_eq!(result.felt, Some("310939249775".to_string()));
    assert_eq!(result.short_string, Some("Hello".to_string()));
}

#[test]
fn test_convert_integration() {
    let result = convert("0x123abc", InputType::Hex).unwrap();
    assert_eq!(result.hex, Some("0x123abc".to_string()));
    assert_eq!(result.felt, Some("1194684".to_string()));

    let result = convert("1194684", InputType::Felt).unwrap();
    assert_eq!(result.hex, Some("0x123abc".to_string()));
    assert_eq!(result.felt, Some("1194684".to_string()));

    let result = convert("Hello", InputType::ShortString).unwrap();
    assert_eq!(result.hex, Some("0x48656c6c6f".to_string()));
    assert_eq!(result.felt, Some("310939249775".to_string()));
    assert_eq!(result.short_string, Some("Hello".to_string()));
}

#[test]
fn test_string_length_limit() {
    let result = convert_from_string("this-is-a-31-character-string..").unwrap();
    assert_eq!(
        result.short_string,
        Some("this-is-a-31-character-string..".to_string())
    );

    let result = convert_from_string("this-is-a-32-character-string...");
    assert!(result.is_err());

    match result {
        Err(e) => assert!(e.contains("String too long")),
        Ok(_) => panic!("Expected an error for string longer than 31 characters"),
    }

    let result =
        convert_from_string("This string is way too long to be a valid short string in Starknet");
    assert!(result.is_err());
}
