use std::io; // input/output library (from standard library)
use rand::Rng; // defines methods that random number genarators implement 

fn main() {
    println!("Guess the number!"); // the ! indicates that println! is a macro and not a function
    
    /* rand::thread_rng - function will give the particular random number generator: one that is
     * local to the current thread of execution and seeded by the operatong system
     * gen_range - method that takes two numbers as arguments and generates a random number between
     * them, a number between 1 and 100*/
    let secret_number = rand::thread_rng().gen_range(1,101);
    
    println!("The secret number in: {}", secret_number); 

    println!("Please input you guess");

    /* let - used to create a variable, in Rust variables are immutable (unchangeble) by default
     * mut - makes the variable mutable 
     * String::new - function that retuns a new instance of a String 
     * String - string type provided ny std library
     * :: - indicates an associated function of the String type (static method, performed on a type
     * and not a particular instance)
     * new - funtion creates a new empty string */
    let mut guess = String::new(); // mutable variable that is currently bound to a new, empty instance of a String
    
    /* stdin - function from std::io module
     * .read_line(&mut guess) - calls the real_line method on the input handle to get input from
     * the user with the argument &mut guess
     * & - indicates that this argument is a reference and is by default immutable, reference is
     * made mutable with &mut guess rather than &guess 
     * .expect() - error handling */
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    /* {} - placeholder */
    println!("You guessed: {}", guess);
}
