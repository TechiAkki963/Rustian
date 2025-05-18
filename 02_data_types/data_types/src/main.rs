#[allow(unused_variables)]

fn main() {
    println!("Data Types");

    let _eight_bit: i8 = 112;
    let _eight_bit_unsigned: u8 = 15;

    // usize
    // isize

    println!("Dear Dwayne,\nHow are u?");
    println!("\t Once upon a time");
    println!("Juilet said \"I love you romeo\"");

    const filepath: &str = r"C:\My Dcoument\new\videos";
    println!("{filepath}");

    let value: i32 = -15;
    println!("{}", value.abs());
    println!("{}", value.pow(2)); // output: 225
    println!("{}", value.pow(3)); // output: -3375

    let empty_space = "    my content     ";
    println!("{}", empty_space.trim()); // output: my content

    // Floating

    let pi: f64 = 3.14159;
    println!("the current value of pi : {}", pi);
    println!("{:.2}", pi); // output: 3.14
    println!("{}", pi.floor()); // output: 3
    println!("{}", pi.ceil()); // output: 4
    println!("{}", pi.round()); // output: 3    

    println!("{pi:.3}");

    // Casting
    // Casting means to convert from one type to another using the as keyword

    let miles_away: i32 = 50;
    let miles_away_i8 = miles_away as i8;
    let mile_away_u8 = miles_away as u8;

    let miles_away = 100.329032;
    let miles_away_f32 = miles_away as f32;
    let miles_away_int = miles_away as i32;

    // Mathematic operators

    let addition = 5 + 3;
    let multiplication = 5 * 3;
    let subtraction = 5 - 3;
    println!("{addition} {multiplication} {subtraction}");

    let division_floor = 5 / 3;

    let division_decimal = 5.0 / 3.0;
    println!("{division_floor} {division_decimal}");

    let remainder = 9 % 2;
    println!("{}", remainder);

    // Augmented Assignment Operator

    let mut year = 2025;
    year += 1;
    println!("{}", year);

    year -= 1;
    println!("{}", year);

    year *= 2;
    println!("{}", year);

    year /= 5;
    println!("{}", year);

    // Boolean

    let is_handsome = true;
    let is_silly = false;

    println!("Handsome: {is_handsome}, Silly :{is_silly}");

    let age: i32 = -40;
    let is_young = age < 35;
    println!("{is_young}"); // true
    println!("{} {}", age.is_positive(), age.is_negative()); // False true

    // Boolean Inversion

    let playing = true;
    println!("{}", !playing); // output : false

    // Equality == =
    // Difference  ||

    // Character TYpe

    let first_initial = 'B';
    // let name = "John";
    let emoji: char = '🎧';
    println!(
        "{} {}",
        first_initial.is_alphabetic(),
        emoji.is_alphabetic()
    );
    println!("{} {}", first_initial.is_uppercase(), emoji.is_uppercase());
    println!("{} {}", first_initial.is_lowercase(), emoji.is_lowercase());

    // ***Array

    let number: [i32; 6] = [4, 8, 15, 16, 23, 42];

    let apples = ["Granny Smith", "Machintosh", "Red Delicious"];
    println!("Length: {}", apples.len());

    let currency_rates: [f64; 0] = [];

    // reading and writing array element

    let seasons = ["Spring", "Summer", "Fall", "Winter"];

    let first = seasons[0];
    let second = seasons[1];

    println!("The first season is {first} and the second season is {second}.");

    // println!("{}",seasons[5]);

    // Mutable Array
    let mut car = ["Ferrari", "Lamborgini", "McLaren", "Pagani"];
    println!("{}", car[2]);
    car[2] = "Porsche";
    println!("{}", car[2]);

    //THe display Trait
    // The display trait display simple datatypes numbers, floats , booleans
    // but do not support complex datatype
    println!("{}", 9);
    println!("{}", 3.14);
    println!("{}", true);

    // println!("{}",car);
    // println!("{}",car);
    // |                   ^^^ `[&str; 4]` cannot be formatted with the default formatter

    // The Debug Trait

    println!("{:?}", car);
    // or
    println!("{car:?}");

    println!("{car:#?}");

    // dbg! Macro or Debug Macro
    // its helps the developer to debug and locate the line of code
    dbg!(2 + 2); // [src/main.rs:164:5] 2+2 = 4  

    dbg!(car);
    //     [src/main.rs:166:5] car = [
    //     "Ferrari",
    //     "Lamborgini",
    //     "Porsche",
    //     "Pagani",
    // ]

    // Tuple
    let employee = ("Molly", 32, "Marketing");

    // let name = employee.0;
    // let age=employee.1;
    // let department = employee.2;

    // or

    let (name, age, department) = employee;

    println!("Name: {name}, Age: {age}, Department: {department}");
    println!("{employee:#?}");

    // Range and Raneg Iteration
    let month_of_days: std::ops::Range<i32> = 1..31;
    println!("{month_of_days:?}");

    let month_of_days = 1..=31;
    println!("{month_of_days:?}");

    for month in month_of_days {
        println!("{month}");
    }

    let letters: std::ops::Range<char> = 'b'..'f';

    for letter in letters {
        println!("{letter}")
    }

    let colors = ["Red", "Yellow", "Blue"];

    for color in colors {
        println!("{color}");
    }

    // Intro to Generic



    /*
Declare an i32 variable assigned to 1337.
Use the underscore character to add a visual
separator between the numbers.
 
Cast the i32 to an i16 integer and assign the result
to a separate variable.
 
Declare a floating-point value of your choosing.
Print out the number with 3 digits of precision.
 
Declare a 'with_milk' variable set to a Boolean.
Declare a 'with_sugar` variable set to a Boolean.
 
Declare a 'is_my_type_of_coffee` variable. It should
be set to true if the coffee has both milk and sugar.
 
Declare an `is_acceptable_coffee` variable. It should
be set to true if the coffee has either milk or
sugar.
 
Declare an array with four i8 integers of your choosing
Print out the array in its Debug representation.
 
Declare a tuple consisting of the integer, float,
a Boolean, and the array that you previously declared.
Print out the tuple in its Debug representation.
*/


let x: i32 = 1_337;

let a: i16 = x as i16;
println!("{a}");

let pi: f64 = 3.145637;
println!("Pi = {pi:.2}");

let arr: [i8; 4] = [2,4,6,8];
dbg!("{arr}");
println!("{arr:#?}");

let tuple = (1,3.14,true,arr);

dbg!("{tuple}");
println!("{tuple:#?}");
}
