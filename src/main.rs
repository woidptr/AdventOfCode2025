mod days;

fn main() {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    // println!("pattern: {:?}, path: {:?}", pattern, path)

    let day: u8 = 1;

    match day {
        1 => {
            let passwords = days::day_01::compute();
            println!("The answer to the puzzle of the first day:");
            println!("Part 1 (regular password): {}", passwords.without_protocol);
            println!("Part 2 (password with protocol 0x434C49434B): {}", passwords.with_protocol);
        },
        2 => {
            println!("The answer to the puzzle of the second day:");
        },
        _ => println!("Not a possible day"),
    }

    // 1182
    // let password = days::day_01::compute();
    // println!("Password: {}", password);
}
