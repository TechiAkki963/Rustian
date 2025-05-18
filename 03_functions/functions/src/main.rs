fn main() {
    open_pizza_store("Newcastle Upon Tyne");
    open_pizza_store("Queens");
    open_pizza_store("Kings");
    bake_pizza(2, "double cheese");
    bake_pizza(15, "Mushroom");

    let result = square(5);
    println!("The square is {result}");

    let result1 = cube(3);
    println!("The cube of the number {result1}");
}

// Parameters and Arguments

fn open_pizza_store(neighbourhood: &str){
    println!("Opening Pizza store in {neighbourhood}");
}

fn bake_pizza (number: i32, topping: &str){
    println!("Baking {number} {topping} pizza.");
}

// Explicit Values

fn square(number: i32) -> i32 {
    return number * number;
}

// Implicit Return Value

fn cube(number: i32) -> i32{
    number * number * number
}

// Unit as a return value


// Scope




fn shoe_size() {
    12
}