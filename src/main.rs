use std::io::{self, stdin, BufRead,};

use crate::helpers::{costperround, tax_per_state, total_cost, States};
mod helpers;

fn main() {

    //Make a variable that holds the chosen state into the match statement
    // Pass that result into the function that picks the tax

    println!("Input the number of boxes:");
    let mut boxes = String::new();
    io::stdin()
        .read_line(&mut boxes)
        .expect("Failed to read line");
    let boxes: f64 = boxes.trim().parse().unwrap();

    println!("Input the number of rounds per box:");
    let mut rounds = String::new();
    io::stdin()
        .read_line(&mut rounds)
        .expect("Failed to read line");
    let rounds: f64 = rounds.trim().parse().unwrap();

    println!("What is the cost per box?");
    let mut price_per_box = String::new();
    io::stdin()
        .read_line(&mut price_per_box)
        .expect("Failed to read line");
    let priceperbox: f64 = price_per_box.trim().parse().unwrap();

    println!("What is the shipping cost?");
    let mut shipping = String::new();
    stdin()
        .read_line(&mut shipping)
        .expect("Failed to read input");
    let shipping: f64 = shipping.trim().parse().unwrap();

    println!("Does the website charge tax?");
    let mut tax = String::new();
    io::stdin()
        .read_line(&mut tax)
        .expect("Failed to read line");

    let mut tax_rate: f64 = 0.0;

    if tax.trim().to_uppercase() == "YES" {
        println!("Enter your state abbreviation:");
        let mut input = String::new();

        std::io::BufReader::new(std::io::stdin())
            .read_line(&mut input)
            .unwrap();
        let which_state = States::from_str(input.trim()).unwrap();

        // USE YOUR RETURN TYPES AND DONT JUST CALL A FUNCTION FOR NO REASON
        tax_rate = tax_per_state(which_state);

        // println!("if statement which state = {}", &which_state);

        // tax_rate = 0.0715;
    }

    let total_rounds = boxes * rounds;

    // Total is being returned here thanks to Josh
    // tax_calc(total, tax_rate);
    let total = total_cost(boxes, priceperbox, shipping, tax_rate);
    costperround(total, total_rounds);
}
