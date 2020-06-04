async fn print_me (value: &str) -> &str {
    println!("Cats {}", value);

    "\
        Async Result will get displayed only if \
        we await for the process to complete.\
    "
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = tokio::runtime::Runtime::new()?;

    let future1 = runtime.spawn(print_me("grow old"));


    // Can't do this, as fn main is not async,
    // Thus have to use runtime.block_on
    // future1.await;

    runtime.block_on(async {
        println!("Output: {}", future1.await?);

        Ok(())
    })
}
