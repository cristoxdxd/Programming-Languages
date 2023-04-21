# Basic Rust program structure
## Functions
Every Rust program must have one function named `main`. The code in the `main` function is always the first code run in a Rust program. 
  
To declare a function in Rust, we use the `fn` keyword.  
After the function name, we tell the compiler how many parameters or arguments the function expects as input.  
Example:
```rust
fn main(){
    println!("Hello World!");
}
```
When a function has input arguments, we name each argument and specify the data type at the start of the function declaration.
```rust
fn goodbye(message: &str) {
    println!("\n{}", message);
}

fn main() {
    let formal = "Formal: Goodbye.";
    let casual = "Casual: See you later!";
    goodbye(formal);
    goodbye(casual);
}
```
When a function returns a value, we add the syntax `-> <type>` after the list of function arguments and before the opening curly bracket for the function body.
```rust
fn divide_by_5(num: u32) -> u32 {
    if num == 0 {
        return 0; // Return early
    }
    num / 5
}

fn main() {
    let num = 25;
    println!("{} divided by 5 = {}", num, divide_by_5(num));
}
```
We can use the `return` keyword at any point in the function to halt execution and send a value back to the caller. If you send back a return value without using the `return` keyword, you <u>**don't end the statement with a semicolon.**</u>
## Macros
- `todo!` macro: used to identify unfinished code in the Rust program
- `println!` macro: displays to the screen or _standard output_ of one or more input arguments expected  
The macro processes the arguments in order. Each instance of curly brackets `{}` inside the text string is replaced with the value of the next argument in the list.
Example:
```rust
fn main() {
    println!("The first letter of the English alphabet is {} and the last letter is {}.", 'A', 'Z');
}
```
## Variables
To declare a variable use the keyword `let` followed by a unique name. The variables are immutable by default in Rust. After a value is bound to a name, you can't change that value. To mutate a value, add the `mut` keyword between `let` and the variable name.
### Shadowing
```rust
// Declare first variable binding with name "shadow_num"
let shadow_num = 5;

// Declare second variable binding, shadows existing variable "shadow_num" 
let shadow_num = shadow_num + 5; 

// Declare third variable binding, shadows second binding of variable "shadow_num"
let shadow_num = shadow_num * 2; 

println!("The number is {}.", shadow_num);
```
## Data types
You must inform the compiler the specific type by using type annotations.
```rust
let number: u32 = 14;
// let variable_name: data_type = value;
```
### Numbers
Length | Signed | Unsigned
---|---|---
8-bit | `i8` | `u8`
16-bit | `i16` | `u16`
32-bit | `i32` | `u32`
64-bit | `i64` | `u64`
128-bit | `i128` | `u128`
architecture-dependent | `isize` | `usize`
**Floating-point:** `f32` & `f64`
### Booleans
The data type keyword is `bool` and the possible values are `true` or `false`.
### Text
**Characters:** The `char` type is specified by enclosing the item in single quotation marks
**Strings:** The keyword to use strings is `&str`. But, not every string can be known at compile time. For this, Rust use `String` as a second string type.

## Data collections
### Tuples
A tuple is a grouping of values of different types collected into one compound value. After a tuple is declared, it can't grow or shrink in size. Elements can't be added or removed. For example:
```rust
let tuple_e = ('E', 5i32, true);
```
The elements in a tuple can be accessed by index position starting from zero.
```rust
println!("Is '{}' the {}th letter of the alphabet? {}", tuple_e.0, tuple_e.1, tuple_e.2);
```
### Structs
A struct is a type that's composed of other types. The elements in a struct are called fields. You can name each field so it's clear what the value means.
- _**Classic structs**_ are the most commonly used. Each field in the struct has a name and a data type. The fields in the struct can be accessed by using the syntax `<struct_name>.<field_name>`.
- _**Tuple structs**_ are similar to classic structs, but the fields don't have names. As with tuples.
- _**Unit structs**_ are most commonly used as markers. <!--I'll learn more about it in Rust's traits feature topic.-->  

The name of a struct type is capitalized
```rust
// Classic struct with named fields
struct Student { name: String, level: u8, remote: bool }
// Tuple struct with data types only
struct Grades(char, char, char, char, f32);
// Unit struct
struct Unit;
```
We must convert from a string literal reference (`&str`) to a `String` type. The standard method is `String::from(&str)` or the `.to_string()` function.
## Compound data
To create an enum type we use the `enum` keyword. Like structs, enum variants can have named fields, but they can also have fields without names, or no fields at all. Like struct types, enum types are also capitalized.
```rust
enum WebEvent {
    // No associated data type or data
    WELoad,
    // With data types
    WEKeys(String, char),
    // An anonymous struct with named fields and their data types
    WEClick { x: i64, y: i64 }
}
```
Also a enum type could define a separate struct for each variant in the enum
```rust
struct KeyPress(String, char);

struct MouseClick { x: i64, y: i64 }

enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }
```
To instantiate an enum we use the `let` keyword to the assignment for each variant. Simple syntax: `<enum>::<variant>`.  
_Use the `{:#?}` syntax to display the enum structure and data in a readable form. Previously, in order to see certain values during the code execution that aren't otherwise viewable in standard output, you need to use `#[derive(Debug)]`_