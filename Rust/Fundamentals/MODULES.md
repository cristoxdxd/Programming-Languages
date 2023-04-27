# Modules

Modules provide a way to organize code and also privacy guarantees to the values, types and methods they contain.

Example of a simplified authentication API:

```rust
mod authentication {
    pub struct User {
        username: String,
        password_hash: u64,
    }

    impl User {
        pub fn new(username: &str, password: &str) -> User {
            User {
                username: username.to_string(),
                password_hash: hash_password(password),
            }
        }

        pub fn get_username(&self) -> &String {
            &self.username
        }

        pub fn set_password(&mut self, new_password: &str) {
            self.password_hash = hash_password(new_password);
        }
    }
    fn hash_password(password: &str) -> u64 {/*...*/}
}

fn main() {
    let user = auth::User::new("user", "password");

    // println!("username: {}", user.username);
    // error: field `username` of struct `auth::User` is private if we try to access it directly
    // instead we can use the getter method
    println!("username: {}", user.get_username());
    // Same with the password
    // println!("password: {}", user.password_hash);
    // error: field `password_hash` of struct `auth::User` is private if we try to access it directly
    // instead we can use the setter method
    let new_password = "even-better-password";
    user.set_password(new_password);
}
```

When the module contents are too big, code navigation becomes more challenging. So we consider moving the module to a separate file.

Let's move the `authentication` module to a separate file `authentication.rs`:

```rust
// src/authentication.rs
pub struct User {
    username: String,
    password_hash: u64,
}

impl User {
    pub fn new(username: &str, password: &str) -> User {
        User {
            username: username.to_string(),
            password_hash: hash_password(password),
        }
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn set_password(&mut self, new_password: &str) {
        self.password_hash = hash_password(new_password);
    }
}

fn hash_password(password: &str) -> u64 {/*...*/}
```

Now the main file `main.rs` looks like this:

```rust
// src/main.rs
mod authentication;

fn main() {
    let user = authentication::User::new("user", "password");

    println!("username: {}", user.get_username());
    let new_password = "even-better-password";
    user.set_password(new_password);
}
```

## Third-party crates

Rust has a package manager called `cargo`. It is used to download, build and manage dependencies of your project.

To add a dependency to your project, you need to add it to the `Cargo.toml` file. You can do it manually or by using the `cargo` command:

```bash
cargo add regex
```

This will add the following line to the `Cargo.toml` file:

```toml
[dependencies]
regex = "1.8.1"
```

Now you can use the `regex` crate in your project:

```rust
// src/main.rs
use regex::Regex;

fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-01"));
}
```
