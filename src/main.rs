use std::io; // input/output library (from standard library)
use std::cmp::Ordering; 
use rand::Rng; // defines methods that random number genarators implement 


fn main() {
    println!("Guess the number!"); // the ! indicates that println! is a macro and not a function
    
    /* rand::thread_rng - function will give the particular random number generator: one that is
     * local to the current thread of execution and seeded by the operatong system
     * gen_range - method that takes two numbers as arguments and generates a random number between
     * them, a number between 1 and 100
     * Rust default type i32 i.e. signed 32 bit number*/
    let secret_number = rand::thread_rng().gen_range(1,101);
    
    println!("The secret number in: {}", secret_number); 
    loop {
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
            .expect("Failed to read line..");

        /* create a new variable named guess that shadows that previous value of guess with a new type
        * trim - method on the String guess that eliminate any whitespace at the beginning and end and
        * also \n (newline)
        * parse - parses a string into some kind of number type, in this cas u32*/
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");
    
        /* {} - placeholder */
        println!("You guessed: {}", guess);

        /* match - is an expresion made up of arms which is the code that will be run if value match of
        * the arm expresion*/
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
