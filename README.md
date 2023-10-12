# AES-128 Whitebox Implementation in Rust

## Overview

This is a Rust implementation of the AES-128 whitebox encryption scheme, inspired by the Chow scheme. This implementation provides a command-line tool for performing white-box AES-128 encryption and decryption on a given input file.

**Note:** This project was developed as a university project, so be aware of potential bugs and limitations.

## Table of Contents

- [Introduction](#aes-128-whitebox-implementation-in-rust)
- [Table of Contents](#table-of-contents)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Features

- AES-128 whitebox encryption and decryption.
- Command-line tool for easy operation.
- Support for both white-box and classic AES-128 modes.
- User-friendly input validation and error handling.
- Highly modular and structured code for educational purposes.

## Getting Started

### Prerequisites

Before you can use this tool, ensure that you have the following prerequisites installed:

- [Rust](https://www.rust-lang.org/): Make sure you have Rust installed on your system.

### Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/your-username/aes-128-whitebox.git
   ```

2. Change to the project directory:

   ```sh
   cd aes-128-whitebox
   ```

3. Build the project:

   ```sh
   cargo build --release
   ```

## Usage

To use the AES-128 whitebox encryption tool, you can run the following command:

```sh
./aes-128-whitebox --mode MODE --action ACTION --file FILE
```

Replace the placeholders with the following values:

- `MODE`: The mode of operation. Choose from `white_box` or `classic`.
- `ACTION`: The action to perform. Choose from `encrypt` or `decrypt`.
- `FILE`: The path to the file you want to operate on.

For example, to encrypt a file using the white-box mode, you can run:

```sh
./aes-128-whitebox --mode white_box --action encrypt --file input.txt
```

Make sure you have the necessary input files and follow the prompts.

## Contributing

If you would like to contribute to this project, feel free to open issues, create pull requests, or reach out to the project maintainer for further guidance.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
