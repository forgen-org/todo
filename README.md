# Forgen: Simple Todo App

This is a Rust project that uses the Cargo build system and Just for task running. The project is structured into several parts: a core application logic, and two client implementations (Axum and Yew).

## Architecture

The project is divided into several parts:

1. [`application`](command:_github.copilot.openRelativePath?%5B%22application%22%5D "application"): This is where the core business logic of the application resides. It contains commands for adding, removing, and completing tasks. These commands are defined in [`commands.rs`](command:_github.copilot.openSymbolInFile?%5B%22application%2Fsrc%2Fcommands.rs%22%2C%22commands.rs%22%5D "application/src/commands.rs").

2. `framework`: This is a set of common utilities and traits used across the application.

3. [`clients`](command:_github.copilot.openRelativePath?%5B%22clients%22%5D "clients"): This directory contains two client implementations of the application:
    - `axum`: This is a server-side implementation using the Axum web framework. The server exposes several endpoints for interacting with the todo list. The main entry point is [`main.rs`](command:_github.copilot.openSymbolInFile?%5B%22clients%2Faxum%2Fsrc%2Fmain.rs%22%2C%22main.rs%22%5D "clients/axum/src/main.rs").
    - `yew`: This is a client-side implementation using the Yew framework for building web applications with Rust and WebAssembly. The main entry point is [`main.rs`](command:_github.copilot.openSymbolInFile?%5B%22clients%2Fyew%2Fsrc%2Fmain.rs%22%2C%22main.rs%22%5D "clients/yew/src/main.rs").

## Getting Started

To start the project, you need to have Rust and Just installed on your machine. If you don't have them, you can install Rust from the [official website](https://www.rust-lang.org/tools/install) and Just from its [GitHub repository](https://github.com/casey/just).

Once you have Rust and Just installed, you can use Just to run tasks defined in the [`Justfile`](command:_github.copilot.openRelativePath?%5B%22Justfile%22%5D "Justfile"). For example, to start the Axum server, you can run:

```sh
just axum
```

And to start the Yew client, you can run:

```sh
just yew
```

Please refer to the [`Justfile`](command:_github.copilot.openRelativePath?%5B%22Justfile%22%5D "Justfile") for more tasks and details.

## Contributing

Contributions are welcome. Please feel free to open an issue or submit a pull request.
