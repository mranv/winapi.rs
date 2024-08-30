<h1 align="center">
<br>
<img src=assets/1_EyBu3xyslFozyfa_UUlK0w.gif height="400" width="600">
<br>
<strong>Hello, Windows!</strong>
</h1>

A simple Windows GUI application written in Rust using the `windows-rs` crate. This project demonstrates how to create a basic Windows application that displays a "Hello, World!" message box.

## Features

- Creates a Windows GUI application using Rust
- Displays a simple message box with "Hello, World!"
- Includes a Windows application manifest for proper Windows integration
- Demonstrates the use of the `windows-rs` crate for Windows API interaction

## Prerequisites

To build and run this project, you need:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- A Windows machine or a Windows target for cross-compilation

## Installation

1. Clone this repository:

   ```
   git clone https://github.com/mranv/windows-hello-world.git
   cd windows-hello-world
   ```

2. Build the project:
   ```
   cargo build --release --target x86_64-pc-windows-msvc
   ```

## Usage

After building the project, you can find the executable at:

```
target/x86_64-pc-windows-msvc/release/windows_hello_world.exe
```

Run the executable by double-clicking it or from the command line:

```
.\target\x86_64-pc-windows-msvc\release\windows_hello_world.exe
```

You should see a message box appear with the text "Hello, World!".

## Project Structure

- `src/main.rs`: The main application code
- `Cargo.toml`: Project configuration and dependencies
- `app.manifest`: Windows application manifest
- `build.rs`: Build script to include the manifest in the executable

## Customization

To customize the message or title of the message box, modify the following lines in `src/main.rs`:

```rust
MessageBoxW(
    None,
    w!("Hello, World!"),
    w!("Windows Hello World"),
    MB_OK | MB_ICONINFORMATION,
);
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [windows-rs](https://github.com/microsoft/windows-rs) - For providing Rust bindings for Windows APIs
- [Rust](https://www.rust-lang.org/) - For being an awesome programming language

## Contact

If you have any questions or feedback, please open an issue in the GitHub repository.

Happy coding!
