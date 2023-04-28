# Test Strategies

## Unit Tests

Simple functions marked with `#[test]` attribute that verify that the code works as intended. Unit tests are the most basic form of testing. They are used to test individual units of code, such as functions, structs, enums, and modules.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_add() {
    assert_eq!(add(2, 2), 4);
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(2, 4), 6);
}
```

It's important to test if a condition will cause a panic. This is done by using `#[should_panic]` attribute.

```rust
#[test]
#[should_panic]
fn test_add_panic() {
    assert_eq!(add(2, 2), 5);
}
```

Some test cases should be ignored. This is done by using `#[ignore]` attribute. This is useful when you are working on a feature and you want to ignore the test cases that are not related to the feature.. This attribute may optionally include a reason for ignoring the test case.

```rust
#[test]
#[ignore = "This test is not ready yet"]
fn test_add_negatives() {
    assert_eq!(add(-2, -2), -4);
}
```

### **The test module**

Most of the time, unit tests are grouped together into a submodule with the `#[cfg(test)]` attribute.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(2, 4), 6);
    }

    #[test]
    #[should_panic]
    fn test_add_panic() {
        assert_eq!(add(2, 2), 5);
    }

    #[test]
    #[ignore = "This test is not ready yet"]
    fn test_add_negatives() {
        assert_eq!(add(-2, -2), -4);
    }
}
```

The `use super::*;` statement imports the parent module into the test module. This allows the test module to access the parent module's functions, structs, enums, and modules.

The `#[cfg(test)]` attribute tells the compiler to only compile the test module when running tests. When running the program normally, the test module will not be compiled. To run the tests, use the `cargo test` command.

## Documentation Tests

 Documentation tests are written in the documentation comments using the `///` syntax. They are used to test the examples in the documentation comments to make sure that they work as intended.

 Documentation comments are written in Markdown and support code blocks in them. There is the structure of a documentation comment:

```rust
/// Generally, the first line is a brief summary describing the function.
///
/// The next lines present detailed documentation. 
/// Code blocks start with triple backticks. The code has an implicit `fn main()` inside and `extern crate <cratename>`,  
/// which means you can just start writing code.
///
/// ```rust
/// let result = basic_math::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
