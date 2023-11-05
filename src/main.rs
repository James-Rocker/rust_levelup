use log::{info, warn};
use learning_rust::{encapsulate, optional_args, print_labeled_measurement};


pub fn main() {
    info!("beep");
    warn!("a warning");

    encapsulate();

    // even though the args are optional, we still need to give a None object
    println!("{}", optional_args(None, None));

    // even if we are returning a string from the function, this must be interpolated
    println!("{}", print_labeled_measurement(6, 'h'))
    // we could pass values with std::env::args, main does not take args and just takes env variables
}