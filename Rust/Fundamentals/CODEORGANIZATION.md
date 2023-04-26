# Code Organization

## Packages

- Contains funtionality within one or more crates
- Includes information about how to build those crates
- Can contain zero or one library crates
- The information is stored in a `Cargo.toml` file

Running the command `cargo new <package_name>` will create a new package with the following structure:

```bash
package_name
├── Cargo.toml
└── src
    └── main.rs
```

A package can contain multiple binary crates by placing them in the `src/bin` directory. 

If a package contains a `src/lib.rs` file and `src/main.rs` file, it has two crates: a library and a binary, both with the same name as the package.

## Crates

- Is a compilation unit in Rust, which is the smallest piece of code that can be compiled into a library or binary
- Once compiled, a crate produces a `.rlib` file (library) or an executable binary
- Contains an implicit, unnamed top-level module

Every project that you create with the `cargo new` command is a crate itself. All third-party Rust code that you can use as dependencies in your project is also, each, a single crate.

To create a library crate, run the command `cargo new --lib <library_name>`. This will create a new crate with the following structure:

```bash
library_name
├── Cargo.toml
└── src
    └── lib.rs
```

## Modules

- Is a (possibly nested) unit of code organization inside a crate
- Modules can contain:
  - `functions`
  - `structs`
  - `enums`
  - `constants`
  - `traits`
  - `impl` blocks
  - type aliases
  - other modules

Example:

```rust
mod math {
    type Complex = (f64, f64);
    pub fn sin(f: f64) -> f64 { /* ... */ }
    pub fn cos(f: f64) -> f64 { /* ... */ }
    pub fn tan(f: f64) -> f64 { /* ... */ }
}

println!("{}", math::cos(45.0));
```

Modules can be used to organize code within a crate into groups for readability and easy reuse. Modules also control the privacy of items, which is whether an item can be used by outside code (public) or is an internal implementation detail and not available for outside use (private).

By deafult all items are private. To make an item public, the `pub` keyword must be used.

```rust
// Declare a private struct
struct Foo;

// Declare a public struct with a private field
pub struct Bar {
    field: i32,
}

// Declare a public enum with two public variants
pub enum State {
    PubliclyAccessibleVariant,
    PubliclyAccessibleVariant2,
}
```
