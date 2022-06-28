use std::io::{self, stdin, BufRead,};

use crate::helpers::{costperround, tax_per_state, total_cost, States};
mod helpers;

fn main() {
    let mut stop = false;
while !stop {
    // Takes an input for the amount of boxes and parses the users input into a float so math can be performed on it!
    println!("Input the number of boxes:");
    let mut boxes = String::new();
    io::stdin()
        .read_line(&mut boxes)
        .expect("Failed to read line");
    let boxes: f64 = boxes.trim().parse().unwrap();

    // Takes an input for the amount of rounds per box and parses the users input into a float so math can be performed on it!
    println!("Input the number of rounds per box:");
    let mut rounds = String::new();
    io::stdin()
        .read_line(&mut rounds)
        .expect("Failed to read line");
    let rounds: f64 = rounds.trim().parse().unwrap();

    // Takes an input for the cost per box and parses the users input into a float so math can be performed on it!
    println!("What is the cost per box?");
    let mut price_per_box = String::new();
    io::stdin()
        .read_line(&mut price_per_box)
        .expect("Failed to read line");
    let priceperbox: f64 = price_per_box.trim().parse().unwrap();

    // Takes an input for the cost of shipping and parses the users input into a float so math can be performed on it!
    println!("What is the shipping cost?");
    let mut shipping = String::new();
    stdin()
        .read_line(&mut shipping)
        .expect("Failed to read input");
    let shipping: f64 = shipping.trim().parse().unwrap();

    // Takes an input from the user on if the website charges sales tax and stores the user response in the tax variable
    println!("Does the website charge tax?");
    let mut tax = String::new();
    io::stdin()
        .read_line(&mut tax)
        .expect("Failed to read line");

        let mut tax_rate: f64 = 0.0;  //declares the tax rate variable which will later be updated depending on user input

    // This is the if statement that checks if the user needs to pay tax or not and handles which state tax to apply
    if tax.trim().to_uppercase() == "YES" {

        println!("Enter your state abbreviation:");
        let mut input = String::new(); //String that holds the users input for their state

        std::io::BufReader::new(std::io::stdin()) // This takes input
            .read_line(&mut input)// Stores the users input into the input variable
            .unwrap();
        input = input.to_uppercase();
        let which_state = States::from_str(input.trim()).unwrap();  //Calls on the from_str function to transform the users input to an enum value 

        // USE YOUR RETURN TYPES AND DONT JUST CALL A FUNCTION FOR NO REASON
        tax_rate = tax_per_state(which_state); // This function passes in the which_state that we used to 


    }

    let total_rounds = boxes * rounds;
    let total = total_cost(boxes, priceperbox, shipping, tax_rate); //Assigning the total value to the value returned from the total_cost function
    costperround(total, total_rounds);

    println!("Would you like to run the program again?");
    let run_again = String::new();
    if run_again.to_uppercase() == "YES" {
      stop = false;
    }
        
    }


}

