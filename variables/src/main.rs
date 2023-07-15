fn main() {
    // defain variable with `let` and `const`
    // `let` is defalut immutable 
    // `const` is immutable
    let x = 5;


    //if we want change value of variable we need to use `mut`
    let mut y = 5;

    //const is always immutable
    //once you create a const you can't change it
    //defain with `const` and uppercase
    const X: u32 = 5;

    //println! to print to console `!` means macro not function call 
    // {} is placeholder for variable
    // print! is same as println! but without newline
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    //shadowing
    y = 6;
    println!("The mutable value of y is: {}", y);
    println!("The value of Y is: {}", X);

}
