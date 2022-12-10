use std::io;
use std::io::Write;

/// Takes in the user input words and returns the completed mad libs string
pub fn mad_lib_generator(
    exclamation: String,
    adverb: String,
    noun: String,
    adjective0: String,
    adjective1: String,
) -> String {
    let first_letter = adjective1.get(0..1);
    let article = match first_letter {
        Some(f) => {
            if f == "a" || f == "e" || f == "i" || f == "o" || f == "u" {
                "an"
            } else {
                "a"
            }
        }
        None => "a",
    };
    
    format!(
        "{}! He said {} as he jumped into his convertable {} and drove off with his {} wife. That was {} {} time",
        exclamation, adverb, noun, adjective0, article, adjective1
    )
}
/// Added this function to let the user input their own words
/// Gathers user input words, passes it to the generator, and returns the string from the generator
pub fn get_words() -> String{
    println!("Enter an exclamation: (EX. wow, woah, cool)");
    std::io::stdout().flush().expect("Failed to flush stdout");
    let mut exclaim = String::new();
    io::stdin()
        .read_line(&mut exclaim)
        .expect("Failed to read from stdin");
    
    exclaim = exclaim.to_ascii_uppercase().trim().to_string();
    
    println!("Enter an adverb: (EX. cheerfully, sadly, swiftly)");
    std::io::stdout().flush().expect("Failed to flush stdout");
    let mut adverb = String::new();
    io::stdin()
        .read_line(&mut adverb)
        .expect("Failed to read from stdin");
    adverb = adverb.trim().to_string();
    
    println!("Enter an noun: (EX. house, robot, skateboard)");
    std::io::stdout().flush().expect("Failed to flush stdout");
    let mut noun = String::new();
    io::stdin()
        .read_line(&mut noun)
        .expect("Failed to read from stdin");
    noun = noun.trim().to_string();

    println!("Enter an adjective: (Describing word)");
    std::io::stdout().flush().expect("Failed to flush stdout");
    let mut adj0 = String::new();
    io::stdin()
        .read_line(&mut adj0)
        .expect("Failed to read from stdin");
    adj0 = adj0.trim().to_string();

    println!("Enter another adjective: ");
    std::io::stdout().flush().expect("Failed to flush stdout");
    let mut adj1 = String::new();
    io::stdin()
        .read_line(&mut adj1)
        .expect("Failed to read from stdin");
    adj1 = adj1.trim().to_string();

    mad_lib_generator(exclaim, adverb, noun, adj0, adj1) 
}