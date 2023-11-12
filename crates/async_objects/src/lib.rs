use std::future::Future;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// looking at threads for background processing work and multi core processing
pub fn thread_spawn() {
    /*
    Spawn two threads to do work.
    Note that when the main thread of a Rust program completes, all spawned threads are shut down,
    whether or not they have finished running. This stops the worker from completing other tasks.

    The calls to sleep, allow a different thread to run
    */
    println!("Starting thread spawn processing");
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}


pub fn thread_join() {
    /*
    Similar to thread_spawn function, but this allows the spawned thread to finish the work because
    this worker is terminated as handle.join() is blocking
    */
    println!("Starting thread join processing");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

// Messaging and concurrency
pub fn messaging_and_thread_ownership() {
    /*
    Rust uses message-sending concurrency, by using "channels" which have transmitters and receivers

    We can interact with this via the mpsc module. mpsc stands for multiple producer, single
    consumer
     */

    // the channel produces a tuple object, one sender, one receiver. This is non-blocking
    let (sender, receiver) = mpsc::channel();

    // cool, let's spawn a thread and send a message from the main thread
    let message = "hi";
    println!("Sending {}", message);

    // using move so the sender object ownership is passed to the spawned thread
    thread::spawn(move || {
        // because this variable is spawned in a subprocess, we can't access it from the main thread
        let val = String::from(message);
        sender.send(val).unwrap();
    });

    // then receive the message in another thread
    let received = receiver.recv().unwrap();
    println!("Got: {}", received);

    /*
    If you uncomment out this code and try to use the sender here, the ownership has been
    transferred to the spawned thread. So we get `borrow of moved value: `sender`
     */
    // sender.send(String::from("beep")).unwrap();
    // let received = receiver.recv().unwrap();
    // println!("Got: {}", received);
}

// futures and concurrent tasks
/*
In Rust, we can use async functions and async blocks. These implement future objects.
These future blocks do nothing until they are awaited

await are special pieces of Rust syntax that make it possible to yield control of the current
thread rather than blocking. This allows other bits of code to run without waiting for something to
complete running.
 */

// `foo()` returns a type that implements `Future<Output = u8>`.
// `foo().await` will result in a value of type `u8`.
async fn foo() -> u8 { 5 }

pub fn future() -> impl Future<Output = u8> {
    // This `async` block results in a type that implements
    // `Future<Output = u8>`.

    async {
        let x: u8 = foo().await;
        x + 5
    }
}

