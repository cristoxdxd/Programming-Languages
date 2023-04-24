# How Rust Manages Memory

## Table of Contents

### [1. Ownership](#ownership)

### [1.1. Scoping rules](#scoping-rules)

### [1.2. Dropping](#dropping)

### [1.3. Move Semantics](#move-semantics)

### [1.4. In functions](#in-functions)

### [1.5. Copying](#copying)

### [2. Borrowing](#borrowing)

### [2.1. Mutable references](#mutable-references)

Rust manages memory without a garbage collector. Therefore, there are no runtime costs associated with memory management. Rust uses a combination of ownership and borrowing to achieve memory safety.

## Ownership

Ownership system is included in Rust. At compile time, the ownership system checks a set of rules to ensure that the ownership features allow your program torun without slowdowns or crashes.

### Scoping rules

In Rust variables, often called "bindings", are valid only within a certain _scope_.

```rust
// `mascot` is not valid and cannot be used here, because it's not yet declared.
{
    let mascot = String::from("ferris");   // `mascot` is valid from this point forward.
    // do stuff with `mascot`.
}
// this scope is now over, so `mascot` is no longer valid and cannot be used.
```

### Dropping

Rust adds a twist to the idea of scopes. Not only does Rust enforce that variables are valid only within a certain scope, but it also enforces that when a variable goes out of scope, its _value_ is dropped.

>For variables of files, the file ends up being closed. For variables that have allocated memory associated with them, the memory is freed.

```rust
{
    let s = String::from("hello");
    // do stuff with `s`.
} // this scope is now over, and `s` is no longer valid. Its value is dropped.
```

### Move Semantics

Sometimes we want to transfer ownership from one binding to another. The simpliest example is when declaring a new binding with an existing binding. The previous binding is no longer valid. In Rust, "transferring ownership" is known as _moving_.

### In functions

When passing a variable to a function, the ownership of the variable is moved to the function.

```rust
fn process(input: String) {}

fn caller() {
    let s = String::from("hello");
    process(s); // `s` is moved to `process`.
    process(s); // Error! `s` is no longer valid here, because it was moved to `process`.
}
```

### Copying

A value that implements the Copy trait, isn't moved but is copied. Simple types like integers implement the `Copy` trait. Therefore, they are copied instead of moved.

We can _explicitly_ copy a value by using the `.clone()` method.

```rust
fn process(input: String) {}

fn caller() {
    let s = String::from("hello");
    process(s.clone()); // `s` is copied to `process`.
    process(s); // `s` is moved to `process`.
}
```

But, it can make the code slower. Avoid to call the `clone` method that makes a full copy of the data. Therefore, it's better to use _references_ in order to "borrow" values.

## Borrowing

Wouldn't it be nice if we could pass a variable to a function without losing ownership of it? We can do that with _borrowing_. Borrowing is the act of creating a reference to an object. References allow us to "borrow" values without losing ownership of them.

```rust
fn process(input: &String) {}

fn caller() {
    let s = String::from("hello");
    process(&s); // `s` is borrowed by `process`.
    process(&s); // `s` is borrowed by `process`.
}
```

By using the reference symbol `&` you can create a reference to a value. The type of the reference is `&T`, where `T` is the type of the value.

### Mutable references

References are immutable by default. But, you can create mutable references by using the `&mut` syntax.

```rust
fn process(input: &mut String) {
    input.push_str(" world");
}

fn caller() {
    let mut s = String::from("hello");
    process(&mut s);
}
```

When we borrow a value of any type, the following rules apply:

- Inmutable references (`&T`) can be borrowed any number of times.
- Mutable references (`&mut T`) can be borrowed only once at a time.

> Two mutable references cannot exist at the same time.  
> A mutable reference cannot exist at the same time as an immutable reference.

Dangling pointers are not possible in Rust. The compiler will ensure that all references are valid.

```rust
fn main() {
    let x;
    {
        let y = 42;
        x = &y; // We store a reference to `y` in `x` but `y` is about to be dropped.
    }
    println!("x: {}", x); // `x` refers to `y` but `y has been dropped!
}
```

In the same example, ther is the lifetime of each variable.

```rust
fn main() {
    let x;                // ---------+-- 'a
    {                     //          |
        let y = 42;       // -+-- 'b  |
        x = &y;           //  |       |
    }                     // -+       |
    println!("x: {}", x); //          |
}                         // ---------+
```

The lifetime could be needed when we have a function that returns a reference.

```rust
fn main() {
    let magic1 = String::from("abracadabra!");
    let magic2 = String::from("shazam!");

    let result = longest_word(&magic1, &magic2);
    println!("The longest magic word is {}", result);
}

fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

It's possible that lifetimes could be different whenever the function is called. We don't know the concrete lifetimes of the references that will be passed to our `longest_word` function, and we can't determine if the reference that will be returned will always be a valid one. So we can add generic lifetime parameters to our function signature.

Whenever a struct or enum holds a reference in one of its fields, we must annotate that type definition with the lifetime of each reference that it carries along with it.

```rust
#[derive(Debug)]
struct Highlight<'document>(&'document str);

fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    println!("{:?}", fox);
    println!("{:?}", dog);
}
```
