fn main() {
    // Mutability:
    let mut x = 5;

    println!("The value of x is: {}", x);

    x = 6;

    println!("The value of x is: {}", x);

    // Shadowing:
    let x = x + 1;

    {
        let x = x + 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of the shadowed x in the outer scope is: {}", x);

    // Shadowing with type transformation:
    let spaces = "   ";
    println!("The value stored in variable _spaces is: {}.", spaces);
    let spaces = spaces.len();
    println!(
        "The new value that is now stored in the variable spaces \
        after it's shadowing is: {}.",
        spaces
    );

    // On the other hand, if the code was written in the following format,
    // we would have gotten a compilation error for mismatched types:
    //
    // let mut spaces = "   ";
    // spaces = spaces.len();
    //
}
