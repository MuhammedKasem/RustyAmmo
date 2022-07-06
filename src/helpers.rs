use std::{f64, io::stdin};

pub enum States {
    AL,
    MT,
    AK,
    NE,
    AZ,
    NV,
    AR,
    NH,
    CA,
    NJ,
    CO,
    NM,
    CT,
    NY,
    DE,
    NC,
    FL,
    ND,
    GA,
    OH,
    HI,
    OK,
    ID,
    OR,
    IL,
    PA,
    IN,
    RI,
    IA,
    SC,
    KS,
    SD,
    KY,
    TN,
    LA,
    TX,
    ME,
    UT,
    MD,
    VT,
    MA,
    VA,
    MI,
    WA,
    MN,
    WV,
    MS,
    WI,
    MO,
    WY,
}

// Login Function that I created. Not really needed for this program but decided to add it to sharpen my skills!
pub fn quick_login() {
    let mut correct_user_name = false;
    let mut correct_password = false;
    let mut stop = false;
    'login: while !stop {
        println!("Enter your username:");
        let mut username = String::new();
        stdin()
            .read_line(&mut username)
            .expect("Failed to read input!");
        println!("Enter your password:");
        let mut password = String::new();
        stdin()
            .read_line(&mut password)
            .expect("Failed to read input!");

        if username.trim().to_uppercase() == "MKASEM" {
            correct_user_name = true;
        }
        if password.trim() == "testPassword" {
            correct_password = true;
        }

        if correct_user_name && correct_password == true {
            stop = true;
            println!("Login Success!")
        } else {
            println!("Login credentials are incorrect! Please try again!");
            continue 'login;
        }
    } // loop end
}

pub fn tax_per_state(state: States) -> f64 {
    match state {
        States::AL => 0.0924,
        States::MT => 0.0000,
        States::AK => 0.0176,
        States::NE => 0.0694,
        States::AZ => 0.0840,
        States::NV => 0.0823,
        States::AR => 0.0947,
        States::NH => 0.0000,
        States::CA => 0.0882,
        States::NJ => 0.0660,
        States::CO => 0.0777,
        States::NM => 0.0784,
        States::CT => 0.0635,
        States::NY => 0.0852,
        States::DE => 0.0000,
        States::NC => 0.0698,
        States::FL => 0.0701,
        States::ND => 0.0696,
        States::GA => 0.0735,
        States::OH => 0.0722,
        States::HI => 0.0444,
        States::OK => 0.0897,
        States::ID => 0.0602,
        States::OR => 0.0000,
        States::IL => 0.0881,
        States::PA => 0.0634,
        States::IN => 0.0700,
        States::RI => 0.0700,
        States::IA => 0.0694,
        States::SC => 0.0744,
        States::KS => 0.0870,
        States::SD => 0.0640,
        States::KY => 0.0600,
        States::TN => 0.0955,
        States::LA => 0.0955,
        States::TX => 0.0820,
        States::ME => 0.0550,
        States::UT => 0.0719,
        States::MD => 0.0600,
        States::VT => 0.0624,
        States::MA => 0.0625,
        States::VA => 0.0575,
        States::MI => 0.0600,
        States::WA => 0.0929,
        States::MN => 0.0749,
        States::WV => 0.0652,
        States::MS => 0.0707,
        States::WI => 0.0543,
        States::MO => 0.0829,
        States::WY => 0.0522,
    }
}

pub fn total_cost(boxes: f64, priceperbox: f64, shipping: f64, tax_rate: f64) -> f64 {
    let total: f64 = (boxes * priceperbox) * (tax_rate + 1.0) + shipping;
    println!("Your total cost is ${:.2}!", total);
    total
}

pub fn costperround(total: f64, totalrounds: f64) {
    let cpr: f64 = total / totalrounds;
    println!("Your cost per round is ${:.2}!", cpr);
}

#[derive(Debug)]
pub struct InputError;

// This matches the users input to an enum of the matching state!
impl States {
    pub fn from_str(s: &str) -> Result<States, InputError> {
        match s {
            "AL" => Ok(States::AL),
            "MT" => Ok(States::MT),
            "AK" => Ok(States::AK),
            "NE" => Ok(States::NE),
            "AZ" => Ok(States::AZ),
            "NV" => Ok(States::NV),
            "AR" => Ok(States::AR),
            "NH" => Ok(States::NH),
            "CA" => Ok(States::CA),
            "NJ" => Ok(States::NJ),
            "CO" => Ok(States::CO),
            "NM" => Ok(States::NM),
            "CT" => Ok(States::CT),
            "NY" => Ok(States::NY),
            "DE" => Ok(States::DE),
            "NC" => Ok(States::NC),
            "FL" => Ok(States::FL),
            "ND" => Ok(States::ND),
            "GA" => Ok(States::GA),
            "OH" => Ok(States::OH),
            "HI" => Ok(States::HI),
            "OK" => Ok(States::OK),
            "ID" => Ok(States::ID),
            "OR" => Ok(States::OR),
            "IL" => Ok(States::IL),
            "PA" => Ok(States::PA),
            "IN" => Ok(States::IN),
            "RI" => Ok(States::RI),
            "IA" => Ok(States::IA),
            "SC" => Ok(States::SC),
            "KS" => Ok(States::KS),
            "SD" => Ok(States::SD),
            "KY" => Ok(States::KY),
            "TN" => Ok(States::TN),
            "LA" => Ok(States::LA),
            "TX" => Ok(States::TX),
            "ME" => Ok(States::ME),
            "UT" => Ok(States::UT),
            "MD" => Ok(States::MD),
            "VT" => Ok(States::VT),
            "MA" => Ok(States::MA),
            "VA" => Ok(States::VA),
            "MI" => Ok(States::MI),
            "WA" => Ok(States::WA),
            "MN" => Ok(States::MN),
            "WV" => Ok(States::WV),
            "MS" => Ok(States::MS),
            "WI" => Ok(States::WI),
            "MO" => Ok(States::MO),
            "WY" => Ok(States::WY),
            _ => Err(InputError),
        }
    }
}

//This is the function that is called upon to take an input from the user and assign it to a specified variable
//in the main function while simultaneously handling errors and repeating the question if an invalid response is recieved.
pub fn get_input() -> f64 {
    let mut valid = false;
    let mut val: f64 = 0.0;
    'check: while !valid {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");
        val = match input.trim().parse() {
            //Parses the users input and declares a match statement
            Ok(num) => {
                //Makes sure the users input is of type f64
                valid = true; //If the users input is of the correct type this breaks the loop
                num //Returns the number
            }
            Err(_) => {
                println!("Invalid input, try again!");
                continue 'check; //This continue allows the program to prompt the user for a response again after an invalid input
            }
        }
    }
    return val;
}
