fn main() {
    // A. Variables and Mutability
    // 1. Shadowing
    // Shadowing is a way to create a new variable with the same name as an existing variable
    // The new variable can have a different type
    // The old variable is not modified, it is shadowed
    // Shadowing is different from mutability
    // Immutable variable
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6; // This would not work because x is immutable
    // Define immutable variable again
    let x = 6;
    println!("The value of x is: {x}");

    // 1.2.
    let z = 5;
    println!("The value of z is: {z}");
    let z = z + 1;
    println!("The value of z is: {z}");

    // 1.3.
    // Shadowing with different type
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
    // But if we try to use mut for this, as the following, it wouldn't work.
    // Because we cannot mutate a variable's type.
    // let mut spaces = "  ";
    // spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    // 2.
    // Mutable variable
    let mut y = 5;
    println!("The value of y is: {y}");
    // Overwrite mutable variable, no need to define it again
    y = 6;
    println!("The value of y is: {y}");

    // 3.
    // Constants
    // Constants are always immutable
    // Constants are defined using the const keyword
    // Constants can be defined in any scope, including the global scope
    // Constants can be defined with any type
    // Constants can be defined with a type annotation
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {MAX_POINTS}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
}
