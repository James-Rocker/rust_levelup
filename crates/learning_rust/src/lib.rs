// as this file is a lib, which means it's a library file meant to be imported
// while if this were a main.rs, it'd be an executable

fn do_a_thing() {
    // rust functions are by default private. That's pretty cool
    println!("Goodbye moon");
}

pub fn encapsulate() {
    // we can make functions public which means the code can be imported elsewhere
    println!("Hello world!");
    do_a_thing();
}

// we weren't using this function within the module so we mark the code as dead so the compiler
// doesn't complain. do_a_thing doesn't need it because it's used within another function
#[allow(dead_code)]
fn another_function(x: i32) -> i32 {
    println!("The value of x is: {}", x);
    return x
}

pub fn print_labeled_measurement(value: i32, unit_label: char) -> String {
    // some string concatenation
    return format!("{}{}", value, unit_label)
}

pub fn optional_args(string_option: Option<String>, second_string_option: Option<String>) -> String {
    // by default, variables are immutable however, we can specify mutability and then pass default values
    let a_thing = string_option.unwrap_or("Boo! You didn't give an argument.".parse().unwrap());
    let found_unit = second_string_option.unwrap_or("Oh no, you also didn't specify the second argument".parse().unwrap());
    format!("{} {}", a_thing, found_unit) // implicit returns are allowed
}

