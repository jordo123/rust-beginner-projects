extern crate rust_beginner_projects;
use rust_beginner_projects::mad_libs;

fn main() {
    println!(
        "{}",
        mad_libs::mad_lib_generator("Woah", "happily", "TV", "smelly", "exciting")
    );
}
