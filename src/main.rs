mod days;

fn main() {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    // println!("pattern: {:?}, path: {:?}", pattern, path)

    let day: u8 = 1;

    match day {
        1 => {
            println!("The answer to the puzzle of the first day (part 1) is: {}", days::day_01::compute());
        },
        2 => {
            println!("The answer to the puzzle of the second day is: ");
        },
        _ => println!("Not a possible day"),
    }

    // 1182
    // let password = days::day_01::compute();
    // println!("Password: {}", password);
}
