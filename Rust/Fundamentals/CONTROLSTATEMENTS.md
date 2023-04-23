# Rust Control Statements

## Arrays

An array is a collection of objects of the same type that are stored sequentially in memory.
An array can be defined in two ways:

- A comma-separated list of values, where the length isn't specified.
- The initial value followed by a semicolon, and then the array length.

```rust
// Declare array, initialize all values, compiler infers length = 7
let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
  
// Declare array, initialize all values to 0, length = 5
let bytes = [0; 5];
```

Access the elements in an array by indexing `<array>[index]`

## Vectors
As with arrays, vectors store multiple values that have the same data type. Unlike arrays, the size or length of a vector can grow or shrink at any time. 
A way to declare and initialize a vector is with the `vec!` macro.

```rust
// Declare vector, initialize with three values
let three_nums = vec![15, 3, 46];
println!("Initial vector: {:?}", three_nums);  
// Declare vector, value = "0", length = 5
let zeroes = vec![0; 5];
println!("Zeroes: {:?}", zeroes);
```

Vectors can also be created by using the `Vec::new()` method.

```rust
// Create empty vector, declare vector mutable so it can grow and shrink
let mut fruit = Vec::new();
```

Use the `push()` method to add a value to the end of the vector.
Use the `pop()` method to remove the value at the end of the vector.  
_Vectors support indexing_

## Conditionals

We can create conditional branches in our code by using the `if` and `else` keywords. Unlike most other languages, if blocks in Rust can also act as expressions. Example:

```rust
let formal = true;
let greeting = if formal { // if used here as an expression
    "Good day to you."     // return a String
} else {
    "Hey!"                 // return a String
};
println!("{}", greeting)   // prints "Good day to you."
```

Multiple `else if` conditions can be used after the starting `if` condition, and before a closing `else` condition, which is optional.
