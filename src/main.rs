use std::io::{self};

fn main() {

    let mut total: f64 = 0.0;

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
    let mut priceperbox = String::new();
    io::stdin()
    .read_line(&mut priceperbox)
    .expect("Failed to read line");
    let priceperbox: f64 = priceperbox.trim().parse().unwrap();

    println!("What is the shipping cost?");
    let mut shipping = String::new();
    io::stdin()
    .read_line(&mut shipping)
    .expect("Failed to read input");
    let shipping: f64 = shipping.trim().parse().unwrap();

    let totalrounds = boxes * rounds;

// total is being returned here thanks to josh
    total = totalcost(boxes, priceperbox, shipping);
    println!("{}", totalrounds);
    costperround(&total, rounds);

    


}

fn totalcost(boxes: f64, priceperbox: f64, shipping: f64) -> f64 {
    let total: f64 = (boxes * priceperbox) + shipping;
    println!("Your total cost is {}!", total);
    total
}

fn costperround(&total: &f64, totalrounds: f64) {
    let cpr: f64 = total / totalrounds;
    println!("Your cost per round is {}", cpr);
}

