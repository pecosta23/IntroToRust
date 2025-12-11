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
    let mut number0 = 7;
    println!("Num0 = {}\n", number0);

    number0 = 1;
    println!("Num0 now equals to = {}\n", number0);

    let number1 = 12;
    println!("Num1 value is = {}\n", number1);

    //condition 0 
    if number0 < 10 {
        println!("Small Number!\n");
    } else {
        println!("Big Numbaa!\n");
    }

    //condition 1
    if number1 < 10 {
        println!("Small Number!\n")
    } else {
        println!("Big Numbaaaaa!\n")
    }

    //lil loop
    for idx in 0..8{
        println!("i = {}\n", idx);
    }

    Ok(())
}
