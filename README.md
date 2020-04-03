# How to call Windows API 

#### A simple example on calling MessageBoxW

Calling a simple Windows API, even as simple as MessageBoxW, requires a lot of work.

What you need to do is:

- There is a crate called `winapi`, which includes nearly all Windows API declarations inside. Add `winapi` to your `Cargo.toml`.

- Look up documentation of `winapi` carefully, and find all **features** that your target API requires to use. For example, to use MessageBoxW, I need to turn on these features: `["winuser", "windef", "winnt"]`. Write these things into your `Cargo.toml`.

- Import required functions into your scope.

```rust
use winapi::{
    shared::windef::HWND,
    um::{
        winuser::{MessageBoxW, MB_ICONINFORMATION, MB_OK},
    }
};
```

- You will need a `win_str` helper function to convert Rust `&str` to Windows *Unicode*(actually UTF-16 LE) string. **NOTICE:** There is also a function named `once` in the `winapi` crate. You don't want to mix up with these things.

```rust
fn win_str(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}
```

- Finally you can call MessageBoxW like a normal function.

```rust
unsafe {
    MessageBoxW(
        0 as HWND,
        win_str("世界！").as_ptr(),
        win_str("ハロー").as_ptr(),
        MB_ICONINFORMATION | MB_OK,
    );
}
```

- The program will show a dialog like:

![Message box with text: Hello world](https://i.imgur.com/DS25bhY.png)