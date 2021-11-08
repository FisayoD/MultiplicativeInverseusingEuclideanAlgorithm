//use keyword is used to shorten the path required to refer to a module item. The core functionality in Rust is provided by a module called std. std::io allows you access input and output functionality.
use std::io;
fn mod_inverse(a: i32, m: i32) {
    // we're trying to change values of x and y in the gcd function so we declare them mutably and pass their references to the function
    //we declare mutable because variables in rust are immutable meaning their values can't be changed if you have used a language like javascript before it would have the same implication as when you use const variables
    let mut x = 0;
    let mut y = 0;
    let g = gcd_extended(a, m, &mut x, &mut y);
    //the if block executes if and only if the value of g is not equal to 1 this means that for a number's multiplicative inverse to exist it must be congruent, it's greatest common divisor must equal 1
    if g != 1 {
        println!("Inverse doesn't exist");
    } else {
        //the res is the multiplicative inverse and is gotten using the formula attached and of course the else will only execute if gcd equals 1
        let res = (x % m + m) % m;
        println!("Modular multiplicative inverse is {}", res);
    }
}

// x and y are of referenced mutable types  
fn gcd_extended(a: i32, b: i32, x: &mut i32, y: &mut i32) -> i32 {
    //the if block executes if and only if the value of a is 0
    if a == 0 {
        // we dereference here to assign to those values in memory.
        // this allows to actually change the values of x and y declared above I think
        *x = 0;
        *y = 1;
        //b is returned meaning the b is what is sent to main program whenever function is called and terminates if the value of a is 0
        return b;
    }

    // since the gcd function takes referenced mutable values, we declare x1 and y1 mutably.
    //the variable x and y also stores results of recursive call
    let mut x1 = 0;
    let mut y1 = 0;
    let gcd = gcd_extended(b % a, a, &mut x1, &mut y1);

    // remember, we dereference with the asterisks so we can assign new values to x and y
    //x and y here use results of recursive call stored in x1 and y1
    *x = y1 - (b / a) * x1;
    *y = x1;
    //gcd is returned meaning the gcd value is what is sent to main program whenever function is called and terminates
    return gcd;
}
pub fn run() {
    //prompt to tell user to enter the number of which they want to find modulus
    println!("please enter a number");
    //initalizes a variable, all variables entered are automatically of string type and have to be converted later
    let mut input1 = String::new();
    //the io module handles input and output and provides related functions, here the line is read and the unwrap simply collects result of computation and if there was an error or no value program execution stops
    io::stdin().read_line(&mut input1).unwrap();
    //prompt to tell user to enter the modulus
    println!("please enter a modulus");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    //i32 means 32 bit signed integer type, it specifies number of integer bits the variable can accept, trim removes white space and parse is used for the type conversion from string to 32 bit signed integer type as specified
    let num: i32 = input1.trim().parse().unwrap();
    let modulus: i32 = input2.trim().parse().unwrap();
    //line of code below is a function call and makes use of values entered by the user
    mod_inverse(num, modulus);
}
