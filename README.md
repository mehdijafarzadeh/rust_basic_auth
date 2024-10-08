# Rust HTTP Client with Basic Authentication Example

This project demonstrates how to make an HTTP GET request using the `reqwest` crate in Rust. It uses basic authentication and performs a simple request to `http://httpbin.org/get` with a username but no password.

## Table of Contents
- [Introduction](#introduction)
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
- [Code Explanation](#code-explanation)
- [License](#license)

## Introduction

This Rust project shows how to make a simple HTTP request using basic authentication, where you can pass a username and optionally a password. It leverages the `reqwest` crate to handle HTTP requests in a blocking manner. In this example, the username is passed but no password is provided.

## Requirements

To run this project, ensure you have the following installed:
- Rust (https://www.rust-lang.org/tools/install)
- Cargo (comes with Rust)

## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/mehdijafarzadeh/rust_basic_auth.git
    cd rust_basic_auth
    ```

2. Add `reqwest` as a dependency in your `Cargo.toml` file:

    ```toml
    [dependencies]
    reqwest = { version = "0.12.8", features = ["blocking"] }
    ```

   Alternatively, add the dependency via Cargo:

    ```bash
    cargo add reqwest --features "blocking"
    ```

## Usage

Run the project with:

```bash
cargo run
