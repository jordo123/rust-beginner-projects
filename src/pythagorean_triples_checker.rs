use std::io::{self, Write};

/// Checks for a pythagorean triple
/// # Examples
/// ```
/// # use rust_beginner_projects::pythagorean_triples_checker::check;
/// assert_eq!(check(Some((3, 4, 5))), true);
/// assert_eq!(check(Some((1, 2, 3))), false);
/// ```
/// # Parameters
/// `nums` - An optional tuple containing the sides of a triangle
/// - `Some(side_a, side_b, hypotenuse)`: Program will check if it is a pythagorean triple and exit right after
/// - `None`: Program will prompt the user for input until the user exits
/// # Returns
/// `bool` - If passed parameters, retuns whether it was a pythagorean triple or not
pub fn check(nums: Option<(i32, i32, i32)>) -> bool {
    println!("Pythagorian Triple Checker");
    // If it is getting the sides from the parameters, exit after one pass of the loop
    // If it is getting the sides from the prompt, continue to loop till the user quits
    let mut from_parameters = false;
    let mut is_triple = false;
    while !from_parameters {
        // Get the sides, either from the parameters, or through the input fuction
        let (side_a, side_b, side_c) = match nums {
            Some(i) => {
                from_parameters = true;
                i
            }
            None => prompt(),
        };
        // a^2 + b^2 == c^2
        if side_a.pow(2) + side_b.pow(2) == side_c.pow(2) {
            println!("It's a Pythagorian Triple");
            is_triple = true;
        } else {
            println!("It's NOT a Pythagorian Triple");
        }
    }
    is_triple
}

/// Gets input from the user
/// # Quits
/// Will exit if it get's a non-integer input
pub fn prompt() -> (i32, i32, i32) {
    let mut side_a = String::new();
    let mut side_b = String::new();
    let mut side_c = String::new();
    let stdin = io::stdin();
    print!("Side A: ");
    io::stdout().flush().expect("Failed to flush the screen");
    // Get input
    stdin
        .read_line(&mut side_a)
        .expect("Failed to read from stdin");
    // Turn side_a into an integer. If it can't, quit
    let side_a: i32 = match side_a.trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Side A is invalid!");
            std::process::exit(1);
        }
    };
    print!("Side B: ");
    io::stdout().flush().expect("Failed to flush the screen");
    stdin
        .read_line(&mut side_b)
        .expect("Failed to read from stdin");
    let side_b: i32 = match side_b.trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Side B is invalid!");
            std::process::exit(1);
        }
    };
    print!("Hypotenuse: ");
    io::stdout().flush().expect("Failed to flush the screen");
    stdin
        .read_line(&mut side_c)
        .expect("Failed to read from stdin");
    let side_c: i32 = match side_c.trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!("The Hypotenuse is invalid!");
            std::process::exit(1);
        }
    };
    (side_a, side_b, side_c)
}
