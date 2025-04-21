fn main() {
    // Uncomment the function you want to run
    count_down();
    loop_label_demo();
    infinite_basic_loop();
}

fn infinite_basic_loop() {
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

fn count_down() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
fn loop_label_demo() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
