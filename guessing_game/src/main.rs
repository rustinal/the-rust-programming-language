// The Rng trait defines methods that random number generators implement, and this trait must be in
// scope for us to use those methods.
use rand::Rng;

// The Ordering type is another enum that is used to represent the result of a comparison.
// It has three variants: Less, Greater, and Equal.
use std::cmp::Ordering;

// To obtain user input and then print the result as output, we need to bring the io
// input/output library into scope with the use statement.
// The io library comes from the standard library, known as std.
// The std::io module is part of the Rust standard library, which provides basic input and output functionality.
// The io module contains types and functions for reading from and writing to the console, files, and other I/O streams.
// In this case, we are using it to read user input from the console.
use std::io;

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

    // The rand::thread_rng() function creates a random number generator that is local to the current thread.
    // The gen_range() method generates a random number within the specified range.
    // The ..= syntax is a range expression that creates a range from 1 to 100, inclusive.
    // The gen_range() method is called on the random number generator created by thread_rng().
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // The loop keyword creates an infinite loop.
    loop {
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

        // Shadow the previous guess variable with a new one.
        // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique
        // ones, guess and guess_string.
        // We bind this new variable to the expression guess.trim().parse().
        // The guess in the expression refers to the previous guess variable, which is a String.
        // The user must press Enter after typing their guess, which adds a newline character to the end of the string.
        // The trim() method is called on the guess string to remove that newline character and any other leading or trailing whitespace.
        // The parse() method on strings converts a string to another type. Here we use it to convert
        // from a string to a number.
        // The colon (:) indicates that we are specifying the type we want to parse the string into.
        let guess: u32 = match guess.trim().parse() {
            // We switch from an expect() call to a match expression to handle the error case.
            // This is similar to the Ordering result of the cmp method.
            // The match expression is used to handle the result of the parse() method.
            // The parse() method returns a Result type, which can be either Ok or Err.
            // The num in Ok(num) is a variable that will hold the parsed number if the parse() method is successful.
            // The => syntax is used to bind the value of the Ok variant to the num variable.
            Ok(num) => num,
            Err(_) => continue,
        };
        // If we want to modify num, say square it, then it gives an error
        // "type annotations needed  cannot satisfy '<_ as std::str::FromStr>::Err == _'"
        // let guess: u32 = match guess.trim().parse::<u32>() {
        //     Ok(num) => num * num,
        //     Err(_) => continue,
        // };

        // The guess variable is now a String that contains the user input.
        // The {} set of curly brackets is a placeholder for the value of the variable guess.
        // When printing the value of a variable, the variable can go inside the curly brackets.
        // When printing the result of evaluating an expression, place empty curly brackets in the
        // format string, then follow the format string with a comma-separated list of expressions to
        // print in each empty curly bracket placeholder in the same order they appear.
        println!("You guessed: {}", guess);

        // A match expression is made up of a series of arms, each of which consists of a pattern and an expression.
        // An arm consists of a pattern to match against, and the code that should be run if the value
        // given to match fits that arm's pattern.
        // When the code compares the guess to the secret number, it will use the cmp() method.
        // Say, the guess is 9 and the secret number is 7. Then the guess is greater than the secret number.
        // The cmp() method will return the Ordering value of Ordering::Greater.
        // The match expression gets the result of the cmp() method and compares it to the three possible arms:
        // Ordering::Less, Ordering::Greater, and Ordering::Equal.
        // It looks at the first arm's pattern, Ordering::Less, and sees that Ordering::Greater doesn't
        // match with Ordering::Less. So it moves on to the next arm.
        // It looks at the second arm's pattern, Ordering::Greater, and sees that Ordering::Greater matches.
        // So it runs the code in that arm, which prints "Too big!" to the console.
        // The match expression ends after the first successful match, so it won't look at the last arm
        // in this scenario.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
        }
    }
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
//
// Crates
// Crates are packages of Rust code that can be shared and reused.
// The project we are working on is a binary crate, which is an executable.
// A binary crate is a standalone executable program that can be run from the command line.
// To get random numbers, we will use the rand crate.
// The rand crate is a library crate, which contains code that is intended to be used in other
// programs and can't be executed on its own.
// The rand crate should be added to the Cargo.toml file in the dependencies section.
