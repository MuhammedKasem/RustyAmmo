use std::{io::{self, stdin, Read}};

fn main() {

    let mut total: f64 = 0.0f64;

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
    io::stdin()
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
        tax_rate = 0.0715;
    }


    let total_rounds = boxes * rounds;

    // Total is being returned here thanks to Josh
    // tax_calc(total, tax_rate);
    total = total_cost(boxes, priceperbox, shipping, tax_rate);
    costperround(total, total_rounds);

    


}

fn total_cost(boxes: f64, priceperbox: f64, shipping: f64, tax_rate: f64) -> f64 {
    let total: f64 = (boxes * priceperbox) * (tax_rate + 1.0) + shipping;
    println!("Your total cost is ${:.2}!", total);
    total
}



fn costperround(total: f64, totalrounds: f64) {
    let cpr: f64 = total / totalrounds;
    println!("Your cost per round is ${:.2}!", cpr);
}

