# Generic Data Types

To implement a generic data type, we need to use the `<>` syntax after the name of the struct, enum, or function.Then we can use the generic type in the definition where we would normally use a concrete type.

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let boolean = Point { x: true, y: false };
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let string_slice = Point { x: "high", y: "low" };
}
```

But we can also define a generic type that takes multiple generic types. For example, we can define a `Point` struct that holds values of two generic types.

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    let string_and_integer = Point { x: "Hello", y: 5 };
    let integer_and_boolean = Point { x: 5, y: true };
}
```

## Shared behavior with traits

A trait is a collection of methods defined for an unknown type: `Self`. They can access other methods declared in the same trait. The Rust standar library provides many useful traits, such us:

- `io::Read` and `io::Write` for reading and writing bytes to a source.
- `Iterator` for types that can produce a sequence of values.
- `Debug` for printing out debugging information.
- `Clone` for creating a copy of an object.
- `ToString` for converting to a `String`.
- `Default` for creating an empty instance of a data type. Like zero for numbers, empty for vectors, and "" for `String`.

But we can also define our own traits. For example, we can define a trait that has one method `area` that calculates the area of a shape.

```rust
trait Area {
    fn area(&self) -> f64;
}
```

Inside the braces, we declare the method signature that describes the behavior of the types that implement this trait.

Then we can implement this trait by creating some new types:

```rust
struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        use std::f64::consts::PI;
        PI * self.radius.powf(2.0)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
```

When a type implements a given trait, we can call the methods defined in the trait on instances of that type. Continuing with the example:

```rust
let circle = Circle { radius: 5.0 };
let rectangle = Rectangle {
    width: 10.0,
    height: 20.0,
};

println!("Circle area: {}", circle.area());
println!("Rectangle area: {}", rectangle.area());
```

### Derive trait

Creating a trait implementation for a type is a lot of work. Fortunately, we can use the `derive` attribute to automatically create some trait implementations for us.

The following traits might be implemented to a successful compilation:

- `Debug` trait to print the instance using `{:?}`.
- `PartialEq` and `Eq` traits to implement equality testing. The `==` and `!=` operators are defined in terms of these methods.
- `PartialOrd` and `Ord` traits to compare and sort instances of this type. The `<`, `<=`, `>`, and `>=` operators are defined in terms of these methods.
- `Display` trait to convert instances of this type to `String` to be formatted with the `{}` specifier. Nevertheless, this trait cannot be derived for a generic type.

```rust
use std::fmt;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main(){
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 4, y: -3 };

    if p1 == p2 {
        println!("equal!");
    } else {
        println!("not equal!");
    }

    println!("{}", p1);
    println!("{:?}", p1);
}
```

### Use trait bounds and generic functions

We can use trait bounds to specify that a generic can be any type that implements the trait we specify.

```rust
#![allow(dead_code, unused_variables)]

trait AsJson {
    fn as_json(&self) -> String;
}

fn send_data_as_json(value: &impl AsJson) {
    println!("Sending JSON data to server...");
    println!("-> {}", value.as_json());
    println!("Done!\n");
}

struct Person {
    name: String,
    age: u8,
    favorite_fruit: String,
}

struct Dog {
    name: String,
    color: String,
    likes_petting: bool,
}

impl AsJson for Person {
    fn as_json(&self) -> String {
        format!(
            r#"{{ "type": "person", "name": "{}", "age": {}, "favoriteFruit": "{}" }}"#,
            self.name, self.age, self.favorite_fruit
        )
    }
}

impl AsJson for Dog {
    fn as_json(&self) -> String {
        format!(
            r#"{{ "type": "dog", "name": "{}", "color": "{}", "likesPetting": {} }}"#,
            self.name, self.color, self.likes_petting
        )
    }
}

struct Cat {
    name: String,
    sharp_claws: bool,
}

fn main() {
    let laura = Person {
        name: String::from("Laura"),
        age: 31,
        favorite_fruit: String::from("apples"),
    };

    let fido = Dog {
        name: String::from("Fido"),
        color: String::from("Black"),
        likes_petting: true,
    };

    let kitty = Cat {
        name: String::from("Kitty"),
        sharp_claws: false,
    };

    send_data_as_json(&laura);
    send_data_as_json(&fido);

    // The Cat type does not implement the trait AsJson.
    // send_data_as_json(&kitty) // uncomment this line to see the compiler error.
}
```

## Iterators

In Rust, all iterators implement a trait named `Iterator` that's defined in the standard library and is used to implement iterators over collections such as ranges, arrays, vectors, and hash maps.

The core of that trait looks like this:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

The `Item` type will be the type returned from the iterator inside the `for` loop block. Creating our own iterators involves two steps: creating a struct to hold the iterator's state and implementing the `Iterator` trait for that struct.

```rust
#[derive(Debug)]
struct Counter {
    length: usize,
    count: usize,
}

impl Counter {
    fn new(length: usize) -> Self {
        Self { length, count: 0 }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        
        self.count += 1;
        if self.count < self.length {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    for number in Counter::new(10) {
        println!("{}", number);
    }

    let sum_until_10: usize = Counter::new(10).sum();
    //assert_eq!(sum_until_10, 55);
    println!("{}", sum_until_10);

    let powers_of_2: Vec<usize> = Counter::new(8).map(|n| 2usize.pow(n as u32)).collect();
    //assert_eq!(powers_of_2, vec![2, 4, 8, 16, 32, 64, 128, 256]);
    println!("{:?}", powers_of_2);
}
```
