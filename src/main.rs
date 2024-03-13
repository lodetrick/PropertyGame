use std::io;
use std::cmp::Ordering;
use rand::Rng;

mod properties;

/*

Maybe guessing game, but you can only use each character once?
Three Final Guesses, where you could use any digit

Get creative where the keyboard letters are put into ascii and translated to a number

Maybe have a fishing minigame using space (scrolling left to right, space when have the right number)

Maybe have a minigame that times the amount of time the user takes to respond, 
 make that time rounded to the nearest second be their input (enter?)

Find out different methods of input

*/

/*
Guess The Number:
The Computer comes up with a random set of properties that a number must fulfill (Maybe 4 of 5)
User Enters a Guess
Computer tells how many properties guess has similar to the correct number
Game ends when the user guesses a number with the correct properties

Maybe the hint is expressed as a bit string, say 10010 that says whether a certain property is being fulfilled
the user doesn't know what the property is, just what type of property it is 
(say, the property that the number is even is the first bit)

Possible Properties (Example, Not Final):
- Prime, Multiple of 4, Multiple of 3, etc
- Number has a 4 somewhere in it
- The left two bits are one
- The Hex representation has an 'A' somewhere in it
- The ascii code represents a letter
- The gcd of the first two digits and second two digits is greater than some number
- The number is a part of a sequence (Fibbonacci, Squares, etc)

Families:
- Factors
- Representation
- Sequences (in or not in a certain sequence)
- # of Digits

Then, there might be multiple goals for each random set: 
Fulfill the most properties
Fulfill certain properties (make the bitstring equal to a certain number)

Possible Meta Puzzles: 
In the start of the game, the user is just given questions and asked to solve them
When the user gets to the meta puzzles, they are given the option to go back to their previous problems
and re-solve them.
Maybe the previous answers are incorporated into a final question that uses the user answer as inputs
- makes the user revisit old questions to fulfill a new goal

Maybe the previous bitstrings are translated into MIPS assembly (opcode i-types)
and the user has to re-answer the previous questions to get the assembler to produce a certain number

Maybe the user has to spell out an ascii-string through the bit inputs 
    (Like have it be "Hello, World!", or something else interesting)

*/

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Again.");
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
