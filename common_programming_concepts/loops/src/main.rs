fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    loop {
        println!("Looping starts...");
        // Let's slow down the loop a bit with a sleep
        // Need to find where code was interrupted
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("Looping...");
        // This loop will run indefinitely
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("This loop will run forever!");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
