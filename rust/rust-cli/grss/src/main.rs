fn main() {
    // Print a sarcastic greeting message to the console
    println!("Oh look, another 'Hello, world!' program. How original.");

    // Get the program name (the first argument in std::env::args())
    // std::env::args() returns an iterator over the command line arguments
    // .next() gets the first item from this iterator (which is the program name)
    // .expect() unwraps the Option, returning the value if Some, or panicking with the message if None
    let program_name = std::env::args()
        .next()
        .expect("Program name can't find I don't know how this will happen");

    // Print the program name with a debug format specifier {:?}
    // Note: There's a missing space between "is" and the format specifier
    println!("Program name is{:?}", program_name);

    // Get the pattern argument (the second argument in std::env::args())
    // std::env::args().nth(1) gets the second item from the arguments iterator
    // .expect() unwraps the Option, returning the value if Some, or panicking with the message if None
    let pattern = std::env::args()
        .nth(1)
        .expect("No pattern given, please give some pattern brother.");

    // Print the pattern with a debug format specifier {:?}
    println!("Pattern name is {:?}", pattern);

    // This is trying to get the path argument, but has the same bug
    // Using .next() a third time on the same iterator still returns the first item
    let path = std::env::args()
        .nth(2)
        .expect("No path given, please give some path brother, what to do?");

    // Print both pattern and path (both are actually the program name due to the bug)
    println!("path is {:?}", path);
}
