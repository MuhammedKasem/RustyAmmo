use std::io::{self, stdin, BufRead,};

use helpers::get_input;

use crate::helpers::{costperround, tax_per_state, total_cost, States};
mod helpers;

fn main() {
    let mut stop = false;
while !stop {

    println!("Input the number of boxes:");
    let boxes = get_input();

    println!("Input the number of rounds per box:");
    let rounds = get_input();

    println!("What is the cost per box?");
    let priceperbox = get_input();

    println!("What is the shipping cost?");
    let shipping = get_input();

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
        tax_rate = tax_per_state(which_state); // This function passes in the which_state variable that we collected from the user
    }

    let total_rounds = boxes * rounds;
    let total = total_cost(boxes, priceperbox, shipping, tax_rate); //Assigning the total value to the value returned from the total_cost function
    costperround(total, total_rounds);

    println!("Would you like to run the program again?");
    let mut run_again = String::new();
    stdin()
    .read_line(&mut run_again)
    .expect("Failed to read input");

    match run_again.trim().to_uppercase().as_str() {
       "YES" => stop = false,
       "NO" => stop = true,
       _ => stop = true,
        
    }
        
    }// while loop end



}

