# Rust Control Statements

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

## Loops

Rust offers three loop expressions to make a program repeat a block of code:

- `loop`: Creates an infinite loop. It repeats, unless a manual stop occurs. The most common way to stop a `loop` expression is by using the break keyword to set a `break` point.
- `while`: Repeat while a condition remains true. A `while` loop begins by evaluating a boolean conditional expression.
- `for`: Repeat for all values in a collection. Uses an iterator.

Examples:

```rust
let mut counter = 1;

let stop_loop = loop {
    counter *= 2;
    if counter > 100 {
        // Stop loop, return counter value
        break counter;
    }
};
println!("Break the loop at counter = {}.", stop_loop);
```

```rust
while counter < 5 {
    println!("We loop a while...");
    counter = counter + 1;
}
```

```rust
let big_birds = ["ostrich", "peacock", "stork"];
for bird in big_birds.iter() {
    println!("The {} is a big bird.", bird);
}
```

We access the items in the collection by using the `iter()` method. Another way to create an iterator is to use the range notation `a..b`.
