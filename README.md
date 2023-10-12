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


## Features

- AES-128 whitebox encryption and decryption.
- Command-line tool for easy operation.
- Support for both white-box and classic AES-128 modes.


## Getting Started

### Prerequisites

Before you can use this tool, ensure that you have the following prerequisites installed:

- [Rust](https://www.rust-lang.org/): Make sure you have Rust installed on your system.

### Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/SandBlastX/aes-128-whitebox.git
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

To change the AES-128 key, you can modify it in the player/build.rs
```rust
    let key: [u8; 16] =  [
        0x34, 0x45, 0xf3, 0x19, 0x5e, 0x27, 0x89, 0x63, 0x96, 0x90, 0x38, 0x41, 0x77, 0x5c, 0xcc,
        0xcf,
    ];
 ```

To use the AES-128 whitebox encryption tool, you can run the following command:

```sh
./player --mode MODE --action ACTION --file FILE
```

Replace the placeholders with the following values:

- `MODE`: The mode of operation. Choose from `white_box` or `classic`.
- `ACTION`: The action to perform. Choose from `encrypt` or `decrypt`.
- `FILE`: The path to the file you want to operate on.

For example, to encrypt a file using the white-box mode, you can run:

```sh
./player --mode white_box --action encrypt --file input.txt
```


