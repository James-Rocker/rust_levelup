use std::fs::File;
use std::path::Path;
use polars::prelude::*;
use parquet::file::reader::{FileReader, SerializedFileReader};

pub fn builtin_data_structs() {
    /*
    Sequences: Vec, VecDeque, LinkedList
    Maps: HashMap, BTreeMap
    Sets: HashSet, BTreeSet
    Misc: BinaryHeap
    Rust docs has some good https://doc.rust-lang.org/std/collections/index.html on when to use which
    */
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    for x in &vec {
        println!("{x}");
    }

}

pub fn basic_data_frame() {
    // this builds a basic dataframe with the Polars
    let default_df = DataFrame::default();
    println!("{}", default_df);

    // Cool, let's now actually build something
    let rust_df = df![
        "Name" => ["Alice", "Bob", "Charlie"],
        "Age" => [25, 30, 22],
        "City" => ["New York", "San Francisco", "Los Angeles"]
    ];

    println!("Rust made {:?}", rust_df);

}

pub fn read_parquet_file() {
    // This is not ideal as the parquet file needs to be in the working directory but will do for now
    let file = File::open(&Path::new("Weather.parquet")).unwrap();

    // serialise and then turn into a mutable iterator obj
    let reader = SerializedFileReader::new(file).unwrap();
    let mut iter = reader.get_row_iter(None).unwrap();

    // Just to give an example of reading the parquet file
    for _ in 1..6 {
        let record = iter.next();
        println!("{:?}", record);
    }

    // if you wanted to do everything in the parquet file then replace lines 29-32 with the below
    // while let Some(record) = iter.next() {
    //     println!("{:?}", record);
    // }
}

pub fn parquet_files() -> PolarsResult<DataFrame> {
    /*
    There is an easier way to read parquet files. Turning it into a more DataFrame object, these are
    2 dimensional data structure that is backed by a Series
     */
    let parquet_df = LazyFrame::scan_parquet("Weather.parquet", ScanArgsParquet::default())?
        .select([
            all()
        ])
        .collect()?;

    println!("Parquet, {:?}", parquet_df);
    Ok(parquet_df)  // we need this because of the Polars result and so we can use the select all
}