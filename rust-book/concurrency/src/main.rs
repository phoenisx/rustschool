#![allow(unused)]
use std::cell::RefCell;
use std::sync::{mpsc, Arc, Mutex};
use std::{env, rc::Rc, thread};
use tokio::{
    sync,
    time::{self, Duration},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let selected_option = args[1].parse::<u8>().expect("Pass a number as argument");
    match selected_option {
        1 => mutex_example(),
        2 => mutex_multithread_example(),
        3 => multithread_spawn(),
        4 => message_passing(),
        _ => {}
    }
}

fn mutex_example() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn mutex_multithread_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

#[tokio::main]
async fn multithread_spawn() {
    let mut start_index = 0;
    while start_index < 3 {
        let handle = tokio::spawn(async move {
            println!("Parent ::::: {start_index}");
            thread::sleep(Duration::from_millis(200));

            let range = 1..5;
            let mut child_handles = Vec::with_capacity(range.len());

            for i in range {
                child_handles.push(tokio::spawn(async move {
                    println!("Child ::::: {start_index}::{i}");
                    thread::sleep(Duration::from_millis(500));
                    format!("Awaiting child complete ::::: {start_index}::{i}")
                }));
            }

            // If I don't wait for spawn's to complete, parent loop will
            // continue to run, without waiting for child processes to complete.
            // This is not desirable, since if parent loop completes, the parent runtime will
            // end, killing all the child processes, irrespective of their completeness.
            for handle in child_handles {
                println!("{}", handle.await.unwrap());
            }
        });

        handle.await.unwrap();
        start_index += 1;
    }
}

fn message_passing () {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for i in (1..5) {
            let val = format!("Hi:::: {i}");
            // When send is done, the `val` value is moved
            // and it's ownership is lost from this thread
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(2000));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

// #[derive(Debug)]
// struct Complex {
//     value: u8
// }

// #[tokio::main]
// async fn multithread_spawn_with_mutex() {
//     let mut start_index = 0;
//     let mut containers = Vec::with_capacity(20);
//     let mut counter = 0;
//     while counter < 20 {
//         containers.push(Arc::new(sync::Mutex::new(Complex { value: counter })));
//         counter += 1;
//     }
//     let containers = Arc::new(sync::Mutex::new(containers));
//     let containers = containers.clone();
//     while start_index < 3 {
//         let handle = tokio::spawn({
//             let first = containers.clone();
//             let first_lock = first.lock().await;
//             let first = first_lock.get(0).unwrap().clone();
//             std::mem::drop(first_lock);
//             async move {
//                 let first = first.lock().await;
//                 println!("Parent ::::: {start_index} :: {}", first.value);

//                 time::sleep(Duration::from_millis(200)).await;

//                 let range = 1..5;
//                 let mut child_handles = Vec::with_capacity(range.len());

//                 for i in range {
//                     child_handles.push(tokio::spawn({
//                         let child = containers.clone();
//                         async move {
//                             let child_lock = child.lock().await;
//                             let child = child_lock.get(i).unwrap().clone();
//                             std::mem::drop(child_lock);
//                             let selected = child.lock().await;
//                             println!("Child ::::: {start_index}::{i} :: {}", selected.value);
//                             time::sleep(Duration::from_millis(2000)).await;
//                             format!("Awaiting child complete ::::: {start_index}::{i}")
//                         }
//                     }));
//                 }
//             }
//         });

//         handle.await.unwrap();
//         start_index += 1;
//     }
// }
