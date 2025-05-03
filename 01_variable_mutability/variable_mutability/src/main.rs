const TAX_RATE: f64 = 7.25;

type Meters = i32;

fn main() {
    println!("Variable and Mutability");

    // Variable and Mutability
    // in Rust, variables are named using snake_case

    let apples_in_garden: i32 = 50;
    let oranges = 14 + 6;
    let fruits = apples_in_garden + oranges;

    println!("{}", fruits);   // output : 70
    println!("{fruits}");     // output : 70
    println!("There are {} fruits in the garden",fruits);  //output :There are 70 fruits in the garden
    println!("There are {fruits} fruits in the garden");  //output :There are 70 fruits in the garden
    println!("There are {} fruits remaining in the garden",fruits -10);  //output : There are 60 fruits remaining in the garden
    println!("there are {0} apples and {1} oranages in the garden and {0} apples are dry", apples_in_garden,oranges); //output: there are 50 apples and 20 oranages in the garden and 50 apples are dry

    // Mutable 

    let mut gym_reps = 10;
    println!("I plan to do {} reps", gym_reps); //output: I plan to do 10 reps

    gym_reps = 15;
    println!("Now I do {} reps", gym_reps);  //output: Now I do 15 reps


    // Variable Shadowing

    let grams_of_protein = "100.35";
    println!("{}",grams_of_protein);
    let grams_of_protein = 100.345;
    println!("{}",grams_of_protein);
    let grams_of_protein = 100;

    println!("{}",grams_of_protein);


    // Scopes 

    //This is a Global Scope

    let cookie_price = 5.99;

    {
        //This is a Nested  Scope
        let cookie_price = 1.99;
        println!("the price of cookie : $ {}",cookie_price); //output: the price of cookie : $ 1.99 
    }

    println!("the price of cookie : $ {}",cookie_price);  // output: the price of cookie : $ 5.99



    // Constants

    // const can never change
    // const can be declare anywhere in the file level
    // const must be CAPITAL
    let income: i32 = 10000;
    println!("The tax rate is {TAX_RATE} and income is {income}"); //output: The tax rate is 7.25 and income is 10000


    // ALIAS

    let miles_of_race: Meters = 1600;
    let two_mile_race_length: Meters = 3200;
    println!("A one mile race is {miles_of_race} meters long and a two mile race is {two_mile_race_length} long"); //output: A one mile race is 1600 meters long and a two mile race is 3200 long


    // Compiler Directive
    #[allow(unused_variables)]
    let idle_variable = 10.00;
    
    #[allow(unused_variables)]
    let idle_variable2 = 10.00;


}

