fn main() {
    println!("Hello, world!");

    function();
    another_function(5);
    print_labeled_measurement(5, 'h');

    let x = five();
    println!("The value of x is: {}.", x);

    let x = plus_one(x);
    println!("After plus_one, the new value of x is: {}.", x);
}

// A function with no parameters:
fn function() {
    println!("Function.");
}

// A function with a single parameter:
fn another_function(x: i32) {
    println!("Another function - the value of x is: {} ", x);
}

// A function with multiple parameters:
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("Print labeld measurement - the measurement is: {}{}.", value, unit_label);
}

// A function with a return value:
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// Since expressions mustn't end with a semicolon - unlike statements, if the function
// was written as follows:
//
// fn plus_one(x: i32) -> i32 {
//      x + 1; // Notice the semicolon at the end here.
// }
//
// then we would have gotten a mismatched types compilation error stating that 
// the function definition declares a return type, yet its body has no tail or return
// expression and therefore it implicitly returns the unit type ().
