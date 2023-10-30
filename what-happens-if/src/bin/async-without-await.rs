use std::{sync::{Arc, Mutex}};



#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let selected = if args.len() > 0 {
        match args[1].parse::<u8>() {
            Ok(val) => val,
            Err(_) => 1
        }
    } else {
        1_u8
    };

    match selected {
        1 => without_await().await,
        2 => multi_handles().await,
        _ => std::process::exit(0),
    }
}


///
/// Can parallel threads inside async blocks be called without await,
/// and if yes does the code inside async runs without awaiting???
///
/// What happens to fn main() ?
/// Does process exits properly?
/// Does Main function waits for the child processes to complete?
///
async fn without_await() {
    for i in 0..10 {
        let moved = i;
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            println!(":::: Thread [{moved}] Complete ::::");
        });
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        // tokio::time::sleep(tokio::time::Duration::from_micros(200)).await;
    }
    // Thread 1 should complete in 500ms
    // Thread 2 should complete in 700ms
    // Thread 3 should complete in 900ms
    // Thread 4 should complete in 1100ms
    // Thread 5 should complete in 1200ms
    // ...
    // Thread 10 should complete in 2300ms
    //
    // Thus the following should at least show output from 1-4 threads
    // tokio::time::sleep(tokio::time::Duration::from_millis(1150)).await;

    // Conclusion:
    // Ok, so one thing can be concluded, outcome when spawned threads are not awaited
    // can be very unexpected. Some threads might drop, while some threads might complete
    // if no tokio::time::sleep is used, the program is too fast to auto await for some threads
    // and quits instantly
}

///
/// Can I spawn threads inside child threads and then later await for all at once??
///
async fn multi_handles() {
    let mut parent_handles = Vec::with_capacity(100);
    let child_handles = Arc::new(Mutex::new(Vec::with_capacity(100)));
    for i in 0..10 {
        let mi = i;
        let cloned = Arc::clone(&child_handles);
        parent_handles.push(tokio::spawn(async move {
            for j in 0..10 {
                let mj = j;
                let handle = tokio::spawn(async move {
                    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                    println!(":::: Thread [{mi}][{mj}] Complete ::::");
                });
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                let mut handle_collection = cloned.lock().unwrap();
                handle_collection.push(handle);
            }
        }));
    }
    // let testing = child_handles.lock().unwrap();
    println!("Collected Parent Handles: {}", parent_handles.len());
    // println!("Collected Child Handles: {}", testing.len());

    for (index, handle) in parent_handles.iter_mut().enumerate() {
        handle.await.expect(&format!("Failed to await parent loop [{index}]"));
    }
}
