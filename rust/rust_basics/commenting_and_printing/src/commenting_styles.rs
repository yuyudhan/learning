// FilePath: src/commenting_styles.rs

//! This is an inner doc comment for the entire module/file
//! It documents the module itself, not individual items within it
//! Inner doc comments must be at the top level or start of an item

/// This is an outer doc comment for the main function
/// It documents the function that follows it
/// Markdown is supported: **bold**, *italic*, `code`
fn main() {
    // This is a single-line comment
    /* This is a block comment
    that spans multiple lines */
    /*
     * This is a block comment with asterisks
     * on each line, which is a common style
     * but not required by Rust
     */
    let x = 5; // Comments can appear at the end of lines
    let y = /* inline block comment */ 10;
    let _result = x + y; // Store result to avoid unused value warning

    /* Block comments can /* be nested */ like this */

    // Call the documented function to demonstrate
    documented_function();

    /*
    Rust doesn't have a built-in way to "comment out" code blocks
    with a keyboard shortcut like some IDEs offer for other languages.
    Instead, you use regular comments or the cfg attribute for
    conditional compilation:
    */

    // Code that is "commented out":
    // fn unused_function() {
    //     println!("This function is commented out");
    // }
}

/// This is an outer doc comment (three slashes)
/// It's used to document functions, structs, etc.
/// Markdown is supported in doc comments!
///
/// # Examples
/// ```
/// documented_function();
/// ```
fn documented_function() {
    //! This inner doc comment is now properly placed
    //! It documents the function body itself
    println!("This function is properly documented!");
}

/// Conditional compilation example
/// This function only exists when running tests
#[cfg(test)]
fn only_compiled_for_tests() {
    // This function only exists when running tests
    println!("Testing mode!");
}

/// This suppresses warnings about unused code
/// Useful for code that's intentionally not called
#[allow(dead_code)]
fn intentionally_unused() {
    // This suppresses warnings about unused code
    println!("I'm intentionally unused!");
}

/// A struct to demonstrate documentation
///
/// This struct shows how to document types
struct ExampleStruct {
    /// This field holds a number
    pub value: i32,
}

impl ExampleStruct {
    /// Creates a new ExampleStruct
    ///
    /// # Arguments
    /// * `value` - The initial value for the struct
    ///
    /// # Returns
    /// A new instance of ExampleStruct
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}
