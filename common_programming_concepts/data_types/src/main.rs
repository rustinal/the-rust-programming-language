fn main() {
    // B. Data Types
    // 1. Scalar Types
    // Scalar types represent a single value
    // There are four primary scalar types in Rust:
    // Integers
    let x: i32 = 5; // 32-bit signed integer
    let y: u32 = 5; // 32-bit unsigned integer
    // i8, i16, i64, i128, isize
    // u8, u16, u64, u128, usize
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    // The isize and usize types depend on the architecture of the machine
    // Integer literals in Rust
    // Decimal
    // Decimal literals are base 10
    // They are the most common type of integer literal
    // They are represented by the i32 type
    let decimal = 98_222; // Decimal
    println!("The value of decimal is: {decimal}");
    // Decimal with underscores
    // Underscores can be used to separate groups of digits
    // This is useful for readability
    // For example, 1_000_000 is easier to read than 1000000
    // Hexadecimal
    let _hex = 0xff; // Hexadecimal
    // Octal
    let _octal = 0o77; // Octal
    // Binary
    let _binary = 0b1111_0000; // Binary
    // Byte (u8 only)
    let _byte = b'A'; // Byte
    // Floating-point numbers
    let z: f64 = 5.0; // 64-bit floating point number
    let w: f32 = 5.0; // 32-bit floating point number
    println!("The value of z is: {z}");
    println!("The value of w is: {w}");
    // Booleans
    // Boolean values can be true or false
    let t: bool = true;
    let f: bool = false;
    println!("The value of t is: {t}");
    println!("The value of f is: {f}");
    // Characters
    // Character type is a single Unicode character
    // It is represented by the char type
    // It is 4 bytes in size
    // It can represent any character in the Unicode standard
    // It is defined using single quotes
    // It can be a letter, number, symbol, or whitespace
    // It can be a special character
    // It can be an emoji
    // It can be a control character
    // It can be a non-ASCII character
    let c: char = 'a';
    let c2: char = '1';
    let c3: char = ' ';
    let c4: char = '!';
    let c5: char = '😊';
    // japanese character
    // This is a Hiragana character
    let c6: char = 'あ';
    // This is a Kanji character
    let c7: char = '𠜎';
    // This is an Odia character
    let c8: char = 'ଓ';
    // This is another Odia character
    let c9: char = 'ଖ';
    println!("The value of c is: {c}");
    println!("The value of c2 is: {c2}");
    println!("The value of c3 is: {c3}");
    println!("The value of c4 is: {c4}");
    println!("The value of c5 is: {c5}");
    println!("The value of c6 is: {c6}");
    println!("The value of c7 is: {c7}");
    println!("The value of c8 is: {c8}");
    println!("The value of c9 is: {c9}");

    // 2. Compound Types
    // Compound types can group multiple values into one type
    // There are two primary compound types in Rust:
    // Tuples
    // Tuples are a fixed-size collection of values
    // They can be of different types
    // They are defined using parentheses
    // They can be destructured
    // They can be indexed
    // They can be iterated over
    // They can be used as function arguments
    // They can be used as function return values
    // They can be used as struct fields
    // They can be used as enum variants
    // They can be used as array elements
    // They can be used as vector elements
    // They can be used as slice elements
}
