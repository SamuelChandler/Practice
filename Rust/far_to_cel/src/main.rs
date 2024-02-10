use std::io::{self, Write};

fn main() {

    println!("Welcome to Farenhiet to Celsius Converter");
    println!("Enter Temperature in Farehiet: ");
    let _ = io::stdout().flush(); //used to force previos print statment out 

    let mut user = String::new();

    io::stdin().read_line(&mut user).expect("failed to read line");

    let far: f32 = user.trim().parse().expect("convert fail");

    let cel:f32 = (far - 32.0)*(5.0/9.0);

    println!("Tempurature in Celsius: {cel} C")
    
}
