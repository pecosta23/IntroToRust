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

    //data types
    let a: i32 = 42;
    let b: f64 = 3.14;
    let c: bool = true;
    let d: char = 'R';
    let e: &str = "Rust\n";

    println!("a, b, c, d, e \nare respectively ->\n
{}, {}, {}, {}, {}", a, b, c, d, e);

    //control flow
    

    Ok(())
}
