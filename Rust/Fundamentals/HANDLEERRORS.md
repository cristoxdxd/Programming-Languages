# Handle Errors

Error handling is the process of anticipating and working with the possibility of failure.

## Table of Contents

### [1. Panic](#panic)

### [2. Option](#option)  

### [2.1. Pattern Matching](#pattern-matching)  

### [2.2. if let Expressions](#if-let-expressions)  

### [2.3. Unwrap & Expect](#unwrap--expect)  

### [3. Result](#result)

## Panic

Panicking is the simplest error handling mechanism in Rust. It is the process of stopping the execution of the program and printing an error message to the console.
Use `panic!` macro to panic.

```rust
fn main() {
    panic!("crash and burn");
}
```

You should use panic! when a program reaches an unrecoverable state. A state where there's absolutely no way to recover from the error.

## Option

Option is an enum defined in the standard library. It is used when a value could be something or nothing. It is used to handle the absence of a value. It's useful for working with values that might exist or that might be empty.  
`Option<T>` manifests itself as one of two variants:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Example:

```rust
let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

let first = fruits.get(0);
println!("{:?}", first);

let third = fruits.get(2);
println!("{:?}", third);

let non_existent = fruits.get(99);
println!("{:?}", non_existent);
```

The attempt to access the 99th item in the vector will return `None` instead of panicking.

### Pattern Matching

Pattern matching is a control flow construct that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.

```rust
let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
for &index in [0, 2, 99].iter() {
    match fruits.get(index) {
        Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
        None => println!("There is no fruit! :("),
    }
}
```

Using a `match` expression against the `Option` value to define a course of action for each of its variants. Rust refers to those branches as _match arms_, and each arm can handle one possible outcome for the matched value.  
You can refine your match expression even further to act differently, depending on the values inside a Some variant.

```rust
let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
for &index in [0, 2, 99].iter() {
    match fruits.get(index) {
        Some(&"coconut") => println!("Coconuts are awesome!!!"),
        Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
        None => println!("There is no fruit! :("),
    }
}
```

Rules of match arms:

- The `match` expression must be _exhaustive_. That means that all possible values of the type must be matched. You can use the `_` placeholder to match all the possible values that aren't specified explicitly.
- The `match` arms are evaluated from top to bottom.

### _if let_ Expressions

`if let` expressions are a shorter way to write `match` expressions when you only care about a single pattern.

```rust
let some_number: Option<u8> = Some(7);
if let Some(7) = some_number {
    println!("That's my lucky number!");
}
```

Instead of a match expression, we can use an `if let` expression and check whether the value inside the `some_number` variable matches the pattern `Some(7)`. If it does, the code inside the block will be executed.

### Unwrap & Expect

`unwrap` is a method that is used to extract the value from an `Option` when you're sure that the value exists. If the value is `None`, `unwrap` will panic.

```rust
let gift = Some("candy");
assert_eq!(gift.unwrap(), "candy");

let empty_gift: Option<&str> = None;
assert_eq!(empty_gift.unwrap(), "candy"); // This will panic!
```

`expect` is similar to `unwrap`, but it allows us to choose the panic message.

```rust
let a = Some("value");
assert_eq!(a.expect("fruits are healthy"), "value");

let b: Option<&str> = None;
b.expect("fruits are healthy"); // panics with `fruits are healthy`
```

These functions are useful when you're prototyping and experimenting with code. Panicking is not a good way to handle errors. You should handle all the possible cases in a way that's appropriate for your situation.

Instead, consider using pattern matching and handle the `None` case explicitly. Otherwise, you can use the `unwrap_or` and `unwrap_or_else` methods to provide default values.

```rust
assert_eq!(Some("dog").unwrap_or("cat"), "dog");
assert_eq!(None.unwrap_or("cat"), "cat");

let k = 10;
assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
assert_eq!(None.unwrap_or_else(|| 2 * k), 20);
```

## Result

By convention, the `Ok(T)` variant represents success and contains a value, and the `Err(E)` variant represents an error and contains an error value.

```rust
enum Result<T, E> {
    Ok(T):  // A value T was obtained.
    Err(E): // An error of type E was encountered instead.
}
```

In contrast to `Option`, which describes the possibility of absence, `Result` is used for recoverable errors.

Also has `unwrap` and `expect` methods, wich either return the value inside the `Ok` variant or panic.

```rust
#[derive(Debug)]
struct DivisionByZeroError;

fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    println!("{:?}", safe_division(9.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0, 2.0));
}
```
