# x12-delimiters

[![Crate](https://img.shields.io/crates/v/x12-delimiters.svg)](https://crates.io/crates/x12-delimiters)
[![API](https://docs.rs/x12-delimiters/badge.svg)](https://docs.rs/x12-delimiters)
[![Build Status](https://github.com/copyleftdev/x12-delimiters/workflows/CI/badge.svg)](https://github.com/copyleftdev/x12-delimiters/actions)
[![Coverage Status](https://coveralls.io/repos/github/copyleftdev/x12-delimiters/badge.svg?branch=main)](https://coveralls.io/github/copyleftdev/x12-delimiters?branch=main)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Dependency Status](https://deps.rs/repo/github/copyleftdev/x12-delimiters/status.svg)](https://deps.rs/repo/github/copyleftdev/x12-delimiters)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)
[![Lines of Code](https://tokei.rs/b1/github/copyleftdev/x12-delimiters)](https://github.com/copyleftdev/x12-delimiters)

A high-performance, zero-copy Rust library for handling X12 EDI delimiters.

## Features

- ⚡ **Fast**: Optimized for performance with benchmarked operations
- 🛡️ **Safe**: Fully tested with unit tests and property-based tests
- 🧩 **Simple API**: Easy-to-use interface for working with X12 delimiters
- 📦 **No dependencies**: Zero runtime dependencies
- 📄 **Well-documented**: Comprehensive documentation
- 🔍 **Validated**: Proper validation of delimiter rules

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
x12-delimiters = "0.1.0"
```

## Usage

```rust
use x12_delimiters::Delimiters;

fn main() {
    // Use default X12 delimiters (segment: ~, element: *, sub-element: :)
    let standard_delimiters = Delimiters::default();
    
    // Create custom delimiters
    let custom_delimiters = Delimiters::new(b'!', b'^', b'&');
    
    // Extract delimiters from an ISA segment
    let isa_segment = b"ISA*00*          *00*          *ZZ*SENDERID       *ZZ*RECEIVERID     *250403*0856*U*00501*000000001*0*P*:~";
    let parsed_delimiters = Delimiters::from_isa(isa_segment).unwrap();
    
    // Access delimiter values
    println!("Segment terminator: {}", parsed_delimiters.segment_terminator() as char);
    println!("Element separator: {}", parsed_delimiters.element_separator() as char);
    println!("Sub-element separator: {}", parsed_delimiters.sub_element_separator() as char);
    
    // Validate delimiter uniqueness
    assert!(parsed_delimiters.are_valid());
}
```

## Performance

The library is optimized for performance:

| Operation | Performance |
|-----------|-------------|
| Delimiters::default | ~898 ps |
| Delimiters::new | ~5.83 ns |
| from_isa_standard | ~1.13 ns |
| from_isa_alternative | ~1.13 ns |
| delimiters_getters | ~673 ps |
| are_valid | ~447 ps |

## X12 EDI Format

X12 is a standard format for electronic data interchange (EDI) documents. In X12, data is organized hierarchically:

1. **Interchange** - Contains one or more functional groups
2. **Functional Group** - Contains one or more transaction sets
3. **Transaction Set** - Contains segments of related data
4. **Segment** - Contains logically related elements
5. **Element** - Contains a single piece of data
6. **Sub-element** - Subdivisions of elements (if needed)

Delimiters define how these structures are separated in the data:
- **Segment Terminator**: Marks the end of a segment
- **Element Separator**: Separates elements within a segment
- **Sub-element Separator**: Separates sub-elements within an element

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
