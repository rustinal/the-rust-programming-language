use std::io;
// To obtain user input and then print the result as output, we need to bring the io
// input/output library into scope with the use statement.
// The io library comes from the standard library, known as std.
// The std::io module is part of the Rust standard library, which provides basic input and output functionality.
// The io module contains types and functions for reading from and writing to the console, files, and other I/O streams.
// In this case, we are using it to read user input from the console.

// By default, Rust has a set of items defined in the standard libray that it brings into the scope
// of every program. This set is called the prelude.
// The prelude includes common types like String, Vec, and Option, as well as traits like Copy and Clone.

// If a type you want to use isn't in the prelude, you can bring it into scope with a use statement.
// The use statement allows you to import items from a module or crate into the current scope.

// The main function is the entry point into the program.
// The fn syntax declares a new function; the parantheses, (), indicate there are no parameters;
// and the left curly bracket, {, starts the body of the function, while the right curly bracket, }, ends it.
fn main() {
    // The println! macro is used to print text or string to the console or screen.
    println!("Guess the number!");
    println!("Please input your guess.");

    // We use the let statement to create a new variable called guess.
    // In Rust, variables are immutable by default, meaning their values cannot be changed once they are set.
    // To create a mutable variable, we use the mut keyword.

    // The = operator is used to assign a value to a variable.
    // It tells Rust we want to bind the value on the right to the variable name on the left.

    // The String::new() function creates a new instance of a String, an empty string, a variable.
    // The String type is a growable, UTF-8 encoded bit of text.
    // It is a growable heap-allocated data structure that represents a sequence of characters.
    // It is used to store and manipulate text in Rust.
    // The :: syntax indicates that new() is a function associated with the String type, not an instance of the String type.
    // An associated function is implemented on a type, in this case String, rather than on an instance of that type.
    // The String::new() function creates a new empty string, which is mutable.
    let mut guess = String::new();
    // If we hadn't imported the io library with use std::io; at the beginnning of the program, we
    // could still the function by writing this function call as std::io::stdin().
    // The stdin() function returns a handle to the standard input stream, which is typically the keyboard.
    // The stdin function returns an instance of the Stdin type, i.e. an instance of std::io::Stdin,
    // which is a type that represents a handle to the standard input for your terminal.
    io::stdin()
        // The read_line() method is called on the Stdin instance returned by stdin().
        // We are passing &mut gyess as an argument to read_line(). to tell it what string to store
        // the user input in. The read_line() method reads a line of input from the standard input
        // stream and appends it to the string passed as an argument.
        // The & indicates that this argument is a reference, which gives you a way to let multiple
        // parts of your code access one piece of data without needing to copy that data into
        // memory multiple times.
        // References are immutable by default, just like variables.
        .read_line(&mut guess)
        // The expect() method is called on the result of read_line(). It is used to handle errors.
        // The read_line() method returns a Result type, which is an enum that can be either Ok or Err.
        // If the read_line() method is successful, it returns Ok with the number of bytes read.
        // If it fails, it returns Err with an error message.
        // If you don't call expect(), the program will compile, but you will get a warning.
        // The right way to suppress such warnings is to handle the error properly.
        // But we just want now to crash this program if it fails, so we call expect().
        .expect("Failed to read line");
    // The guess variable is now a String that contains the user input.
    // The {} set of curly brackets is a placeholder for the value of the variable guess.
    // When printing the value of a variable, the variable can go inside the curly brackets.
    // When printing the result of evaluating an expression, place empty curly brackets in the
    // format string, then follow the format string with a comma-separated list of expressions to
    // print in each empty curly bracket placeholder in the same order they appear.
    println!("You guessed: {}", guess);
}

// More on the String type:
// The String type is mutable, meaning you can change its contents after it has been created.
// The String type is part of the Rust standard library and is defined in the std::string module.
// The String type is commonly used for tasks such as reading user input, storing text data, and manipulating strings.
// The String type is created using the String::new() function, which creates an empty string.
// You can also create a String from a string literal using the to_string() method or the String::from() function.
// The String type is often used in conjunction with the &str type, which is a string slice that represents a borrowed reference to a string.
// The &str type is immutable, meaning you cannot change its contents after it has been created.
// The &str type is commonly used for string literals and for passing strings to functions that do not need to modify them.
