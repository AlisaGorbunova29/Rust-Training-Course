// This chapter is dedicated to the common programming concepts, like variables and their
// mutability, data types, functions and control flow stuff

// MUTABILITY
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function that declares a mutable integer variable, assigns it the value 5, then changes
// it to 10, and prints both values.
#[allow(dead_code)]
pub fn simple_mutability() {
    let mut x: i32 = 5;
    println!("{x}");
    x = 10;
    println!("{x}");
}

// DATA TYPES
// ================================================================================================

// ----- 2 --------------------------------------
// Create variables of types `i32``, `f64``, `bool``, and `char``, assign them values, and print
// them.
#[allow(dead_code)]
pub fn simple_data_types() {
    let x1:i32 = 1;
    println!("{x1}");
    let x2:f64 = 2.3;
    println!("{x2}");
    let x3:bool = true;
    println!("{x3}");
    let x4:char = 'a';
    println!("{x4}");

}

// FUNCTIONS
// ================================================================================================

// ----- 3 --------------------------------------
// Write a function `square` that takes a `u32` integer and returns its square as `u32`.

// IMPLEMENT HERE:

pub fn square(x1: u32) -> u32 {
    x1 * x1
}

// ----- 4 --------------------------------------
// Write a recursive function `factorial` that computes the factorial of a number (n!) as `u32`.

pub fn factorial(n: u32) -> u32 {
    let mut answer:u32 = 1;
    for k in 1..n+1 {
        answer *= k;
    }
    answer
}

// IMPLEMENT HERE:

// CONTROL FLOW
// ================================================================================================

// ----- 5 --------------------------------------
// Write a program that prints whether a provided signed integer number is positive, negative, or
// zero using `if` statement.
pub fn sign_checker(number: i32) -> &'static str {
    if number > 0 {
        "positive"
    } else if number < 0 {
        "negative"
    } else {
        "zero"
    }
}

// ----- 6 --------------------------------------
// Write a program that finds the largest number in an array of 5 integers using a for or while
// loop.
pub fn find_biggest_number(some_array: [u32; 5]) -> u32 {
    
    let mut max_number:u32 = some_array[0];
    for i in 1..5 {
        if some_array[i] > max_number {
            max_number = some_array[i];
        }
    }
    max_number
}
