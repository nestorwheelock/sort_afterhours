
# Afterhours File Sorter

**Afterhours File Mover** is a Rust-based command-line tool that sorts files modified during a time of day for instance "afterhours" (8 PM to 9 AM) to a designated `afterhours/` directory. The tool checks the modification times of files in a directory and relocates files based on the time range.

## Features

- Detects files modified between 8 PM and 9 AM ("afterhours").
- Moves files to a designated `afterhours/` directory.
- Handles both the current directory or a specified directory as input.
- Automatically creates the `afterhours/` directory if it doesn't exist.

## Requirements

- Rust (for building the tool)
- The `chrono` crate for handling dates and times.

## Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/your-username/afterhours-mover.git
   cd afterhours-mover
   ```

2. **Build the project using Cargo:**
   ```bash
   cargo build --release
   ```

## Usage

1. **Move files from the current directory:**
   Run the program without arguments to move files from the current directory:
   ```bash
   ./target/release/afterhours-mover
   ```

2. **Move files from a specific directory:**
   You can also specify a directory as an argument:
   ```bash
   ./target/release/afterhours-mover /path/to/directory
   ```

3. **Afterhours Directory:**
   The files modified between 8 PM and 9 AM will be moved to an `afterhours/` directory created in the current working directory.

## Example

Assume you have a directory with the following files:

```text
file1.txt  (modified at 7 PM)
file2.txt  (modified at 10 PM)
file3.txt  (modified at 8 AM)
```

Running the following command:

```bash
./target/release/afterhours-mover
```

The tool will move `file2.txt` and `file3.txt` to the `afterhours/` directory because they were modified during "afterhours."

## Project Structure

- **`main.rs`**: Contains the core logic that handles detecting afterhours file modifications and moving files to the designated directory.
- **`Cargo.toml`**: Lists the required dependencies for the project, including the `chrono` crate for date and time management.

## License

This project is licensed under the GNU GPLv3 License. See the [LICENSE](LICENSE) file for more details.
