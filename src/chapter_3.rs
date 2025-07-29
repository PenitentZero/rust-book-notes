use std::any::Any;

pub fn _run() {
    //Change a variable  without making it mutable!
    let x: u8 = 12;
    let x: u8 = x + 1;
    let x: u8 = x * 10;
    println!("The value of X: {}", x);

    //Floating type
    let floating_point: f64 = 3.14;
    println!("P: {}", floating_point);

    //Change variables type
    let space = "    ";
    let space = space.len();
    println!("space len: {}", space);

    //Numeric operation
    let _product = 5 * 25;
    let _difference = 3.14 - 0.14;
    let _sum = 24 + 25;
    let _remainder = 59 % 4;

    //Boolean type
    let _boolean: bool;

    //Character type
    let _x = 'H';

    //Tuple
    let tuple: (i8, f32, u8) = (-120, 3.14, 25);
    println!("{}", tuple.0);

    //Array
    let array: [i8; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", array);
    /*Rust is a statically typed language
    It means compiler must know all the types of the variables at the compile time

    Scalar types.

    Integer Types:
    i8 i16 i32 i64 i128 and for unsigned integer u8 u16 u32 u64 u128

    Floating-Point types:
    f32 and f64(default)

    Compound types
    Tuples and arrays
    Tuple
    every index has a specific type
    Array
    Fixed size, every index must have the same type

    Function
    Snake case
    fn
    pub
    Function parameter
    Functions with return values "->"
    Return value
    Return type
    Control flow
    if expression
    condition in if expression must be bool!
    Handling Multiple Conditions with else if
    Repetition with loops
    Loop, while, for
    Break
    Looping Through a Collection with for

    */
}

pub fn celsius_to_fahrenheit(mut temperature: i32, measure: &str) {
    if measure == "C" {
        temperature = (temperature * (9 / 5)) + 32;
        println!("temperature in fahrenheit: {}", temperature);
    } else if measure == "F" {
        temperature = (temperature - 32) * 9 / 5;
        println!("temperature in Celsius: {}", temperature);
    } else {
        println!("Unknown measure format, Try C or F");
    }
}
