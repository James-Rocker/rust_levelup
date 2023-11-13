# rust_levelup

This is where I am housing my rudimentary Rust code while I try 
to learn the language. 

## Rust Setup

### Installation on Mac

Installed with the rustup toolchain via  
`brew install rustup`

Then to initialise the rust compiler  
`rustup-init`

Finally, to verify
`rustc --version`

### Installation on Windows

Install the exe via the [windows-64 bit exe](https://win.rustup.rs/x86_64)

## Project structure

I started this without knowing the proper structure for a rust lang 
project so each domain of learning has their own crate in the crate
folder. For future reference, follow the rust-lang project structure
https://doc.rust-lang.org/cargo/guide/project-layout.html

## Running commands

Code is compiled via cargo. Package is defined in the root `cargo.toml`
config file.

To run the basic main code use the default executable with  
`cargo run --package rust_levelup --bin rust_levelup`

to run other executables, cargo recognises the executables in `bin`
so use  
`cargo run --package rust_levelup --bin async_main`
