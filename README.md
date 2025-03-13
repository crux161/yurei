# Yurei - Binary Visualizer

Yurei is a command-line tool written in Rust that visualizes binary files. It provides a textual representation of the binary data, making it easier to understand the structure and content of files.

## Todo

- **Hexadecimal Representation:** Displays the binary data in hexadecimal format.
- **ASCII Interpretation:** Attempts to interpret and display printable ASCII characters alongside the hexadecimal representation.
- **Offset Display:** Shows the file offset for each line of data.
- **Customizable Output:** Options to control the number of bytes displayed per line.

## Features 
* **Cross-Platform:** Built with Rust, ensuring compatibility across various operating systems.

## Installation

### Prerequisites

* Rust and Cargo installed. If you don't have them, you can install them from [rustup.rs](https://rustup.rs/).

### Building from Source

1.  Clone the repository:

    ```bash
    git clone <repository_url>
    cd yurei
    ```

2.  Build the project using Cargo:

    ```bash
    cargo build --release
    ```

3.  The executable will be located in `target/release/yurei`.

### Installing from Cargo (if published)

* If yurei is published on crates.io, you can install it using:

    ```bash
    cargo install yurei
    ```

## Usage

```bash
yurei <file_path> [options]
```
