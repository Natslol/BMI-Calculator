use whoami;
use std::io;
use std::{thread, time::Duration};

fn main() {
    println!("Hello {}, I will calculate your BMI!\n\nSo let's Start now :)\n ", whoami::username());
    let mut weight = String::new();
    println!("Your Weight (kg): ");
    io::stdin().read_line(&mut weight).expect("Can't read line");
    weight.pop();
    if weight.contains("\r") { weight.pop(); }
    let weight = weight.parse::<f32>().unwrap();
    let mut height = String::new();
    println!("Your Height (cm): ");
    io::stdin().read_line(&mut height).expect("Can't read line");
    height.pop();
    if height.contains("\r") { height.pop(); }
    let height = height.parse::<f32>().unwrap();
    let BMI: f32 = weight / f32::powi(height / 100.0, 2) ;
    if BMI < 18.5 {
        println!("\nYou have {}, you are underweight!", BMI)
    } else if BMI < 24.9 {
        println!("\nYou have {}, you have a normal weight!", BMI)
    } else if BMI < 29.9 {
        println!("\nYou have {}, you are overweight!", BMI)
    } else if BMI > 30.0 {
        println!("\nYou have {}, you are obese", BMI)
    }
    thread::sleep(Duration::from_millis(8000))
}
