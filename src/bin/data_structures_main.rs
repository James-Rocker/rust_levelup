use std::env;
use std::path::PathBuf;
use polars::prelude::*;
use data_structures::{builtin_data_structs, basic_data_frame, read_parquet_file, parquet_files};

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    // needed this to locate the current working directory (for the parquet file to go)
    env::current_dir()
}

fn main() {
    builtin_data_structs();

    println!("Printing a very basic Dataframe first");
    basic_data_frame();

    println!("We need to know the working directory so we can read the file");
    println!("{:?}", get_current_working_dir());

    println!("Let's read from a parquet file");
    read_parquet_file();

    println!("That's useful but let's try using the library polar to make it into a DataFrame");

    let result_df: Result<DataFrame, PolarsError> = parquet_files();
    println!("cool, now we can work with this. Let's unwrap and print the first 5 rows");
    println!("{}", result_df.unwrap().head(Some(5)))
}
