// FilePath: src/struct.rs

// In programming, a struct is a composite data type that groups related
// values together under a single name. The term comes from the concept
// of organizing or structuring data in a meaningful way. Structs allow
// programmers to create custom data types that represent real-world entities
// by bundling related properties together.

//! Comprehensive guide to Rust structs
//! This file demonstrates different types of structs, their usage,
//! and best practices in Rust programming

/// A basic struct representing a person
/// Structs are custom data types that group related data together
/// They're similar to classes in other languages but without inheritance
struct Person {
    name: String, // Owned string data - String type has full ownership of its memory,
    // controls its data exclusively, and is responsible for cleanup when dropped

    // COMMENT: An owned String in Rust has full ownership of its memory allocation,
    // meaning it's responsible for allocating memory when created and deallocating
    // when dropped. In contrast, an unowned reference like &str is just a view into
    // existing string data (either static or owned by something else) without owning
    // the memory itself. Unowned references borrow data temporarily without taking
    // ownership responsibility.
    age: u32, // 32-bit unsigned integer - Primitive type stored directly in the struct
    // (no ownership concerns as it's a Copy type that's stored by value)
    email: String,   // Another owned string
    is_active: bool, // Boolean flag
}

/// A struct with different field visibility
/// Fields are private by default - use `pub` to make them public
///
/// VISIBILITY EXPLAINED:
/// - Struct itself can be public (`pub struct`) or private (no `pub`)
/// - Each field can independently be public (`pub field`) or private
/// - Public struct with private fields = can create instance, can't access
///   private fields directly
/// - Private struct = can't be used outside the module at all
pub struct Rectangle {
    pub width: f64,        // Public field - accessible from outside module
    pub height: f64,       // Public field - accessible from outside module
    area_calculated: bool, // Private field - only accessible within module
}

/// A struct that is not public but has public fields
/// This demonstrates an unusual visibility pattern
/// The struct can only be created within this module, but if exposed through
/// a function or method, its public fields can be accessed from outside
struct PrivateStructPublicFields {
    pub id: u32,           // Public field
    pub name: String,      // Public field
    internal_data: String, // Private field
}

// TODO: Use this also in the program
impl PrivateStructPublicFields {
    /// Constructor - can only be called within this module
    fn new(id: u32, name: String, internal_data: String) -> Self {
        PrivateStructPublicFields {
            id,
            name,
            internal_data,
        }
    }

    /// Public method to access private field
    pub fn get_internal_data(&self) -> &str {
        &self.internal_data
    }
}

/// Example of a private struct (no `pub` keyword)
/// This struct can only be used within this module/file
struct PrivateData {
    secret: String,
}

/// Tuple struct - like a struct but with numbered fields instead of names
/// Useful when you want type safety but don't need named fields
struct Point(f64, f64, f64); // x, y, z coordinates

/// Unit struct - no fields, useful for implementing traits
/// Takes up zero memory but can have behavior
///
/// UNIT STRUCT EXPLAINED:
/// - Contains no data fields (hence "unit")
/// - Primarily used as a marker type for implementing traits
/// - Zero memory overhead - optimized away at compile time
/// - Often used for type safety, state representation, or trait implementations
/// - Can be instantiated with just its name: `let marker = Marker;`
/// - Useful in type-driven design where you need a unique type but no data
/// - Common in API design for type-state patterns or phantom types
struct Marker;
// TODO: Also give a real world exmaple of using a unit struct.

/// Implementation block for Person struct
/// This is where we define methods and associated functions
///
/// IMPL BLOCKS EXPLAINED:
/// - A struct doesn't always need an impl block - it can exist as just data
/// - Simple "plain old data" (POD) structs can be used without methods
/// - Add an impl block when you want to:
///   1. Define methods that operate on struct instances (using self)
///   2. Create associated functions (like constructors)
///   3. Implement traits for your struct
/// - You can have multiple impl blocks for the same struct
/// - Impl blocks can be in different files/modules from the struct definition
/// - Methods provide encapsulation and behavior to your data structures
/// - Associated functions (no self parameter) are like static methods
impl Person {
    /// Associated function (like static method) - creates new Person
    /// Notice: no `self` parameter means this is an associated function
    ///
    /// # Arguments
    /// * `name` - The person's name
    /// * `age` - The person's age
    /// * `email` - The person's email address
    ///
    /// # Returns
    /// A new Person instance
    pub fn new(name: String, age: u32, email: String) -> Self {
        Person {
            name,            // Shorthand for name: name
            age,             // Shorthand for age: age
            email,           // Shorthand for email: email
            is_active: true, // Default value
        }
    }

    /// Method that takes immutable reference to self
    /// Can read data but cannot modify it
    ///
    /// THE `&self` EXPLAINED:
    /// - `&` means "borrow" or "reference" - we don't take ownership
    /// - `self` refers to the current instance of the struct
    /// - `&self` = immutable borrow - we can read but not modify
    /// - This allows multiple readers at the same time
    /// - Original owner keeps ownership, we just "look at" the data
    pub fn greet(&self) -> String {
        format!(
            "Hello, my name is {} and I'm {} years old",
            self.name, self.age
        )
    }

    /// Method that takes mutable reference to self
    /// Can modify the struct's data
    ///
    /// MUTABILITY EXPLAINED:
    /// - In Rust, everything is immutable by default for safety
    /// - `mut` keyword makes things mutable (changeable)
    /// - `&mut self` = mutable borrow - we can read AND modify
    /// - Only ONE mutable reference allowed at a time (prevents data races)
    /// - Can't have immutable and mutable references simultaneously
    pub fn deactivate(&mut self) {
        self.is_active = false;
        println!("{} has been deactivated", self.name);
    }

    /// Method that takes ownership of self
    /// Consumes the struct - can't be used after this call
    ///
    /// OWNERSHIP EXPLAINED:
    /// - `self` (without &) takes ownership of the struct
    /// - The struct is "consumed" - original variable becomes invalid
    /// - This is useful for cleanup operations or transformations
    /// - After calling this method, you can't use the original variable
    pub fn delete(self) -> String {
        format!("{} has been permanently deleted", self.name)
    }

    /// Getter method for private field access
    /// Since `is_active` is private, we need a method to read it
    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

/// Implementation for Rectangle
impl Rectangle {
    /// Constructor for Rectangle
    pub fn new(width: f64, height: f64) -> Self {
        Rectangle {
            width,
            height,
            area_calculated: false,
        }
    }

    /// Calculate area - demonstrates mutable method
    pub fn area(&mut self) -> f64 {
        self.area_calculated = true; // Modifying private field
        self.width * self.height
    }

    /// Check if it's a square
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}

/// Implementation for Point tuple struct
impl Point {
    /// Create new point
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point(x, y, z) // Note: using tuple syntax, not named parameters
    }

    /// Get distance from origin
    pub fn distance_from_origin(&self) -> f64 {
        // Access tuple fields with .0, .1, .2 etc.
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    /// Get coordinates as tuple
    pub fn coordinates(&self) -> (f64, f64, f64) {
        (self.0, self.1, self.2)
    }
}

/// Implementation for PrivateData to demonstrate private struct usage
impl PrivateData {
    /// Even though struct is private, we can have public methods
    pub fn new(secret: String) -> Self {
        PrivateData { secret }
    }

    pub fn reveal_secret(&self) -> &str {
        &self.secret
    }
}

/// Demonstrate visibility rules with examples
fn demonstrate_visibility() {
    println!("\n=== Visibility Examples ===");

    // 1. Public struct with mixed field visibility
    let mut rect = Rectangle::new(5.0, 10.0);

    // ✅ Can access public fields directly
    println!("Width: {}, Height: {}", rect.width, rect.height);
    rect.width = 7.0; // Can modify public field

    // ❌ This would cause compile error - private field:
    // println!("Area calculated: {}", rect.area_calculated);
    // rect.area_calculated = true;

    // ✅ But can access private field through public method
    let area = rect.area(); // This modifies the private field internally
    println!("Area: {}", area);

    // 2. Private struct - can only be used within this module
    let private_data = PrivateData::new(String::from("Top Secret"));
    println!("Secret: {}", private_data.reveal_secret());

    // Note: If this struct were in another module, we couldn't even
    // create an instance of PrivateData
}

/// Demonstrate mutability concepts
fn demonstrate_mutability() {
    println!("\n=== Mutability Examples ===");

    // 1. Immutable variable and struct
    let person = Person::new(String::from("Alice"), 25, String::from("alice@example.com"));

    // ✅ Can call immutable methods
    println!("{}", person.greet());

    // ❌ This would cause compile error - can't call mutable method:
    // person.deactivate();

    // 2. Mutable variable and struct
    let mut mutable_person = Person::new(String::from("Bob"), 30, String::from("bob@example.com"));

    // ✅ Can call both immutable and mutable methods
    println!("{}", mutable_person.greet());
    mutable_person.deactivate(); // This works because variable is `mut`

    // 3. Multiple borrows demonstration
    let name_ref1 = &mutable_person.name; // Immutable borrow
    let name_ref2 = &mutable_person.name; // Multiple immutable borrows OK
    println!("Name refs: {}, {}", name_ref1, name_ref2);

    // ❌ This would cause compile error - can't have mutable borrow
    // while immutable borrows exist:
    // mutable_person.deactivate();
}

/// Demonstrate the :: syntax (scope resolution operator)
fn demonstrate_scope_resolution() {
    println!("\n=== Scope Resolution (::) Examples ===");

    // THE `::` SYNTAX EXPLAINED:
    // :: is called the "scope resolution operator" or "path separator"
    // It's used to access items within namespaces/modules/types

    // 1. Associated functions (like constructors)
    let person = Person::new(
        // Type::function
        String::from("Charlie"), // Type::function
        28,
        String::from("charlie@example.com"),
    );

    // 2. Accessing methods vs associated functions
    println!("{}", person.greet()); // instance.method() - uses dot
                                    // vs
                                    // Person::greet(&person);         // Type::method(&instance) - equivalent

    // 3. Standard library examples
    let vector = Vec::new(); // Vec::new() - associated function
    let string = String::from("Hi"); // String::from() - associated function
    let number = i32::MAX; // i32::MAX - associated constant

    println!("Empty vector: {:?}", vector);
    println!("String: {}", string);
    println!("Max i32: {}", number);

    // 4. Module paths (if we had modules)
    // std::collections::HashMap::new()  // module::submodule::Type::function

    println!(":: separates namespaces, modules, and types from their items");
}

/// Demonstrate struct update syntax and destructuring
fn demonstrate_advanced_features() {
    println!("\n=== Advanced Struct Features ===");

    // Creating struct with field init shorthand
    let name = String::from("Alice");
    let age = 30;
    let email = String::from("alice@example.com");

    let person1 = Person {
        name,  // Shorthand for name: name
        age,   // Shorthand for age: age
        email, // Shorthand for email: email
        is_active: true,
    };

    // Struct update syntax - create new struct using existing one
    let person2 = Person {
        name: String::from("Bob"),
        email: String::from("bob@example.com"),
        ..person1 // Copy remaining fields from person1
                  // Note: person1 is moved here because String doesn't
                  // implement Copy trait
    };

    println!("Person 2: {}", person2.greet());

    // Destructuring - extract fields from struct
    let Person { name, age, .. } = person2; // .. ignores remaining fields
    println!("Extracted: name={}, age={}", name, age);

    // Tuple struct destructuring
    let point = Point::new(1.0, 2.0, 3.0);
    let Point(x, y, z) = point;
    println!("Point coordinates: x={}, y={}, z={}", x, y, z);
}

/// Main function demonstrating struct usage
fn main() {
    println!("=== Rust Structs Tutorial ===\n");

    // 1. Creating structs using associated functions (constructors)
    println!("1. Creating structs:");

    // `mut` EXPLAINED:
    // - `mut` makes the variable mutable (changeable)
    // - Without `mut`, the variable is immutable by default
    // - This allows us to call methods that modify the struct later
    let mut person = Person::new(
        String::from("John Doe"),
        25,
        String::from("john@example.com"),
    );

    let mut rectangle = Rectangle::new(10.0, 5.0);
    let point = Point::new(3.0, 4.0, 0.0);

    // 2. Using methods
    println!("2. Using methods:");
    println!("{}", person.greet());
    println!("Rectangle area: {}", rectangle.area());
    println!(
        "Point distance from origin: {:.2}",
        point.distance_from_origin()
    );

    // 3. Accessing public fields directly
    println!("\n3. Accessing fields:");
    println!(
        "Rectangle width: {}, height: {}",
        rectangle.width, rectangle.height
    );
    println!("Is person active? {}", person.is_active());

    // 4. Modifying structs
    println!("\n4. Modifying structs:");
    person.deactivate(); // Mutable method - requires `mut` variable
    rectangle.width = 15.0; // Direct field access (public field)

    // 5. Tuple struct usage
    println!("\n5. Tuple struct:");
    let (x, y, z) = point.coordinates();
    println!("Point coordinates: ({}, {}, {})", x, y, z);

    // 6. Unit struct
    println!("\n6. Unit struct:");
    let _marker = Marker; // Creates unit struct instance
    println!("Marker created (takes zero memory)");

    // 7. Pattern matching with structs
    println!("\n7. Pattern matching:");
    match rectangle.is_square() {
        true => println!("Rectangle is a square"),
        false => println!("Rectangle is not a square"),
    }

    // 8. Consuming struct (takes ownership)
    println!("\n8. Consuming struct:");
    let deletion_msg = person.delete(); // person is consumed here
    println!("{}", deletion_msg);
    // person can't be used after this point!

    // 9. Advanced features
    demonstrate_advanced_features();

    // 10. Visibility examples
    demonstrate_visibility();

    // 11. Mutability examples
    demonstrate_mutability();

    // 12. Scope resolution examples
    demonstrate_scope_resolution();

    println!("\n=== Key Takeaways ===");
    println!("STRUCTS:");
    println!("• Group related data together into custom types");
    println!("• Three types: named fields, tuple, and unit structs");
    println!("• Struct itself can be pub/private, fields independently too");

    println!("\nVISIBILITY:");
    println!("• `pub struct` = can be used outside module");
    println!("• `pub field` = field accessible outside module");
    println!("• Private fields need getter/setter methods for external access");

    println!("\nMUTABILITY:");
    println!("• Variables immutable by default - use `mut` to change");
    println!("• `&self` = immutable borrow (read-only)");
    println!("• `&mut self` = mutable borrow (read + write)");
    println!("• `self` = takes ownership (consumes the struct)");

    println!("\nSCOPE RESOLUTION (::):");
    println!("• `Type::function()` = associated function (like constructor)");
    println!("• `instance.method()` = method call on instance");
    println!("• `module::Type::item` = access items in namespaces");
}
