use rand::seq::SliceRandom;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

/// A list of responses from the magic 8 ball (10 positive responses, 5 neutral, and 5 negative)
pub static RESPONSES: [&'static str; 20] = [
    "It is certain.",
    "It is decidedly so.",
    "Without a doubt.",
    "Yes - definitely.",
    "You may rely on it.",
    "As I see it, yes.",
    "Most likely.",
    "Outlook good.",
    "Yes.",
    "Signs point to yes.",
    "Reply hazy, try again.",
    "Ask again later.",
    "Better not tell you now.",
    "Cannot predict now.",
    "Concentrate and ask again.",
    "Don't count on it.",
    "My reply is no.",
    "My sources say no.",
    "Outlook not so good.",
    "Very doubtful.",
];

/// Responds to the user after calling `prompt` to get input
pub fn shake() {
    println!("Magic 8 Ball");
    loop {
        // Get input from the user
        let question = prompt();
        // Quit of it is "q"
        if question == "q" {
            break;
        } else if question == "" {
            println!("No Question specified!");
            continue;
        }
        print!("Thinking");
        // Because standard output is line buffered (text is printed to screen
        // when a new line is added to the buffer), `print!` might not print unless
        // the screen is flushed manually
        std::io::stdout().flush().expect("Failed to flush stdout");
        for _ in 0..3 {
            print!(".");
            std::io::stdout().flush().expect("Failed to flush stdout");
            // Sleep to simulate thinking
            thread::sleep(Duration::from_millis(500));
        }
        println!("");
        // Create the random number generator
        let mut rng = rand::thread_rng();
        // Choose one of the responses to use
        let response = RESPONSES.choose(&mut rng).unwrap();
        println!("Magic 8 Ball says: {}", response);
    }
}

/// Prompts the user for input
/// # Returns
/// `String` - The input from the user
pub fn prompt() -> String {
    println!("type q to quit");
    print!("Your Question? > ");
    std::io::stdout().flush().expect("Failed to flush stdout");
    let mut question = String::new();
    io::stdin()
        .read_line(&mut question)
        .expect("Failed to read from stdin");
    question.trim().to_lowercase()
}
