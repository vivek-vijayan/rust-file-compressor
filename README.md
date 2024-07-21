# Rust File Compressor

## Overview

Welcome to **Rust File Compressor**! This Rust application is designed to compress files within a specified directory. It takes two command-line arguments: the file name and the path where the compressed result will be stored.

## Features

- **File Compression**: Efficiently compresses a specified file.
- **Custom Storage Path**: Allows you to choose where the compressed file will be saved.

## Getting Started

### Prerequisites

- Rust installed on your machine. If you don't have Rust installed, you can get it from [the official website](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/vivek-vijayan/rust-file-compressor.git
   ```

2. Navigate to the project directory:
   ```bash
   cd rust-file-compressor
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

### Usage

Run the application with two arguments: the file name to compress and the path where the compressed result will be stored. For example:

```bash
./target/release/rust-file-compressor <file_name> <output_path>
```

Replace `<file_name>` with the name of the file you want to compress and `<output_path>` with the desired path for the compressed file.

### Example

```bash
./target/release/rust-file-compressor input.txt /path/to/output/compressed_file.zip
```

### License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Feel free to open issues or submit pull requests if you have suggestions or improvements. Contributions are always welcome!

## Contact

For any questions or feedback, you can reach out to me:

- **Developer**: Vivek Vijayan
- **GitHub**: [vivek-vijayan](https://github.com/vivek-vijayan/)
