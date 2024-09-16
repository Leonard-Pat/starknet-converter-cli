# Starknet Converter CLI

A command-line interface tool for converting between different data types commonly used in the Starknet blockchain ecosystem.

## Features

- Convert between hexadecimal, felt (field element), and string representations
- Easy-to-use CLI with intuitive options
- Robust error handling and informative error messages
- Support for various input formats

## Usage

The basic syntax for using the Starknet Converter CLI is:

```
snconvert <INPUT> [OPTIONS]
```

Where `<INPUT>` is the value you want to convert, and `[OPTIONS]` are the conversion options you want to apply.

### Options

- `-x, --hex`: Display the hexadecimal representation
- `-f, --felt`: Display the felt (field element) representation
- `-s, --string`: Display the string representation

If no options are specified, the tool will display all available representations.

### Examples

1. Convert a hexadecimal value:

   ```
   snconvert 0x123 -f -s
   ```

2. Convert a felt value:

   ```
   snconvert 1234567890 -x -s
   ```

3. Convert a string:

   ```
   snconvert "Hello, Starknet!" -x -f
   ```

4. Display all representations:
   ```
   snconvert 0xabcdef
   ```

## Error Handling

The Starknet Converter CLI includes robust error handling to help you identify issues with your input:

- Invalid input format
- String too long (max 31 characters for Starknet short strings)
- Non-ASCII characters in string input
- Invalid hexadecimal format

If an error occurs, the tool will display a helpful error message explaining the issue.

## Contributing

Contributions to the Starknet Converter CLI are welcome! Please feel free to submit pull requests, create issues, or suggest new features.

---

For installation instructions, please see the [Installation](#installation) section above.