/// Generate a madlib
/// # Examples
/// ```
/// # use rust_beginner_projects::mad_libs::mad_lib_generator;
/// let m = mad_lib_generator("Cool", "sadly", "Wall", "stinky", "exciting");
/// assert_eq!(m, "Cool! He said sadly as he jumped into his convertable Wall and drove off with his stinky wife. That was an exciting time");
/// ```
/// # Parameters
/// `exclamation` - An exclamation (Waoh, Cool, Shoot)
///
/// `adverb` - An adverb (happily, sadly)
///
/// `noun` - A noun (tree, rock, trash can)
///
/// `adjective0` - An adjective (exciting, kind, brave)
///
/// `adjective1` - A second adjective
///
/// # Returns
/// `String` - The completed madlib
pub fn mad_lib_generator(
    exclamation: &str,
    adverb: &str,
    noun: &str,
    adjective0: &str,
    adjective1: &str,
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
        exclamation, adverb, noun, adjective0, article, "exciting"
    )
}
