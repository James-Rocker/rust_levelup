use std::env;
use log::{info, warn, error};
use env_logger;
use learning_rust::{encapsulate, optional_args, print_labeled_measurement};

fn env_testing(environment_name: &str, environment_value: &str) -> String {
    // This function is pointless mechanically but useful for learning rust :)
    env::set_var(environment_name, environment_value);
    println!("Successfully written to {}", environment_name);

    // env:var returns as result<string, varError> but using unwrap we can just get the value
    let read_env_var = env::var(environment_name).unwrap();
    // although if the value isn't found then this will cause an error after compiling
    println!("Environment key is: {} val is: {}", environment_name, read_env_var);

    // we can use expect if we know what should be there. Useful for testing mainly
    let _read_env_var_expect = env::var(environment_name).expect(environment_value);

    return read_env_var
}

// because this is an executable, rust expects a main function (can be pub or private)
fn main() {
    // the log level must be set before the logger is initialised
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // using the log dependency defined in the root cargo config file
    info!("logging to info");
    warn!("logging a warning");
    error!("logging an error"); // by default, all logging is disabled except exception

    // We declare this variable and never use it. Due to the prefix underscore rust never complains
    let _unused_variable = "thinking";

    // using an imported local library function
    encapsulate();

    // even though the args are optional, we still need to give a None object
    println!("{}", optional_args(None, None));

    // even if we are returning a string from the function, this must be interpolated
    // we could pass values with std::env::args, main does not take args, just takes env variables
    println!("{}", print_labeled_measurement(6, 'h'));

    // setting the args. However, we want to set them to environment variables
    let string_option= "string_option";
    let string_option_val = "beep";
    let second_string_option= "second_string_option";
    let second_string_option_val = "boop";

    // cool, now let's take these variables, set them to the environment, read them back and print
    // the joined function
    println!("{}", optional_args(Option::from(env_testing(string_option, string_option_val)),
                                 Option::from(env_testing(second_string_option, second_string_option_val))));
}
