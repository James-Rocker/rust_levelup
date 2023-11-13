#[allow(unused_imports)]  // in case we want to comment out bits of the imports to run here
use async_objects::{thread_spawn, thread_join, messaging_and_thread_ownership, future};
use tokio::time::{sleep, Duration};

// Uncomment this out if you want to run it. This might be moved later, haven't decided
// fn main() {
//     // thread_spawn();
//     // thread_join();
//     // messaging_and_thread_ownership();
// }

async fn async_task(duration: u64) -> Result<&'static str, &'static str> {
    sleep(Duration::from_secs(duration)).await;
    Ok("Task Completed")
}

#[tokio::main]
async fn main() {
    /*
    if we want to use an async function in the main, we need to spawn a blocking task. The
    library Tokio can do this by
     */
    let val = future().await;
    println!("{}", val);

    /*
    this is not that useful because just one function is being run. However, we can run multiple
    tasks all at once
     */

    let result = tokio::try_join!(
        // Notice, we can't use named args. Everything is positional
        async_task(2),
        async_task(1),
        async_task(3),
    );

    match result {
        Ok((result_1, result_2, result_3)) => {
            println!("{}", result_1);
            println!("{}", result_2);
            println!("{}", result_3);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
}
