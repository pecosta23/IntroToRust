use reqwest;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    //hello
    println!("Hello, world!\n");

    //imutable and mutable
    let x = 10;
    let mut y = 3;
    println!("x = {}, y = {}\n", x, y);
    y = 5;
    println!("y = {}\n", y);

    Ok(())
}
