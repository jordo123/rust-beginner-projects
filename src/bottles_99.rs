/// Calls the `print_bottles` function in a loop and prints the lyrics until it reaches 0
/// # Parameters
/// `bottles` - how many bottles to start with
/// # Preconditions
/// `bottles` must be positive
pub fn sing(bottles: u32) {
    // Goes from bottles to 0
    for i in (0..=bottles).rev() {
        println!("{}", print_bottles(i));
    }
}
/// Returns the line to the song "99 Bottles" given a number of bottles
/// # Examples
/// ```
/// # use rust_beginner_projects::bottles_99::print_bottles;
/// assert_eq!(
///    print_bottles(31),
///    String::from("31 bottles of beer on the wall.\n\
///                  31 bottles of beer. Take one down, pass it around.\n\
///                  30 bottles of beer on the wall.")
/// );
///```
/// # Parameters
/// `bottles` - the number of bottles left on the wall
/// # Preconditions
/// `bottles` must be positive
/// # Returns
/// `String` - The line in the song
pub fn print_bottles(bottles: u32) -> String {
    // Once there are 2 bottles left, when you "Take one down, pass it around"
    // you will have "1 bottle" left on the wall. (instead of "1 bottles")
    if bottles == 2 {
        return format!(
            "{} bottles of beer on the wall.\n\
             {} bottles of beer. Take one down, pass it around.\n\
             {} bottle of beer on the wall.",
            bottles,
            bottles,
            bottles - 1
        );
    // Once there is 1 bottle left, when you "Take one down, pass it around"
    // you will have "0 bottles" left on the wall
    } else if bottles == 1 {
        return format!(
            "{} bottle of beer on the wall.\n\
             {} bottle of beer. Take one down, pass it around.\n\
             {} bottles of beer on the wall!",
            bottles,
            bottles,
            bottles - 1
        );
    // Once we're at zero, there isn't much else we can take down
    } else if bottles == 0 {
        return String::from("Lets do it again!");
    // Until 2, this is what will be printed
    } else {
        return format!(
            "{} bottles of beer on the wall.\n\
             {} bottles of beer. Take one down, pass it around.\n\
             {} bottles of beer on the wall.",
            bottles,
            bottles,
            bottles - 1
        );
    }
}
