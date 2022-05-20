pub fn encapsulate() {
    println!("Hello, world!");
    do_a_thing();
}

fn do_a_thing() {
    // rust functions are by default private. That's pretty cool
    println!("Hello, a different thing!");
}

// we aren't using this function so commenting it out so rust compiler doesn't complain
#[allow(dead_code)]
pub fn another_function(x: i32) -> i32 {
    println!("The value of x is: {}", x);
    return x
}

pub fn print_labeled_measurement(value: i32, unit_label: char) -> String {
    return format!("{}{}", value, unit_label)
}

pub fn optional_args(string_option: Option<String>, second_string_option: Option<String>) -> String {
    // by default, variables are immutable however, we can specify mutability and then pass default values
    let a_thing = string_option.unwrap_or("aghhhh, you didn't give an argument. ".parse().unwrap());
    let found_unit = second_string_option.unwrap_or("oh no, you also didn't specify the second argument".parse().unwrap());
    format!("{}{}", a_thing, found_unit) // implicit returns
}


