fn main() {
    another_function(5, 'h');
    expression();
    let x = five();
    println!("The value of x is: {x}");
    let y = plus_one(x);
    println!("The value of y on plus_one(x) is: {y}");
}

fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expression() {
    let y = {
        let x = 3;
        x + 1 // This is an expression, so it doesn't need a semicolon
        // This is a statement, so it needs a semicolon
    };
    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
