fn main() {
    // Generate the nth Fibanacci number based on user input
    let mut input = String::new();
    println!("Enter the position of the Fibanacci number you want to find:");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: u32 = input.trim().parse().expect("Please enter a valid number");
    let result = fibonacci(n);
    println!("The {}th Fibonacci number is:{}", n, result);
}

// Function to calculate the nth Fibonacci number
fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let mut a = 0;
        let mut b = 1;
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        b
    }
}
