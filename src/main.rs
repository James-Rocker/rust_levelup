mod lib;


fn main() {
    lib::encapsulate();

    // even though the args are optional, we still need to give a None object
    println!("{}", lib::optional_args(None, None));

    // even if we are returning a string from the function, this must be interpolated
    println!("{}", lib::print_labeled_measurement(6, 'h'))
    // we could pass values with std::env::args, main does not take args and just takes env variables
}
