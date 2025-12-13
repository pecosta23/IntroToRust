// std is to read user input
use std::io;

fn main(){
    println!("Type any whole number\n");
    //num 1
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a: i32 = a.trim().parse().unwrap();

    println!("Please type any of these operators: + - * /\n");
    //operator
    let mut ope = String::new();
    io::stdin().read_line(&mut ope).unwrap();
    let ope: char = ope.trim().parse().unwrap();

    println!("Type other whole number\n");
    //num 2
    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();
    let b: i32 = b.trim().parse().unwrap();

    let result = match ope {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => {
            println!("Invalid operator\n");
            0
        }
    };

    println!("Result: {}", result)
}

