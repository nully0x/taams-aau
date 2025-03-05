# Adekunle Ajasin University - Akungbe Journal of Educational Technology

A web application for managing and publishing academic journals.

## Prerequisites

- Rust (latest stable version)
- Cargo
- Git
- Devbox(for linux only)

## Setup

1. Clone the repository:
```bash
git clone https://github.com/nully0x/aau-ajet.git
cd aau-ajet
```
> If you don't want to install rust, cargo or any other dev-dependencies, you can install devbox and it will install them in an isolated environment.
> This is suitable for Linux and MacOs

- Install devbox via `curl -fsSL https://get.jetify.com/devbox | bash`
- Start devbox in the project's root with `devbox shell`

> then you can skip to [Development](#Development) as the neccessary dev-dependencies has been provided in devbox.

2. Install development dependencies:
```bash
cargo install --locked bacon
```

3. Create a `.env` file in the project root:
```bash
RUST_LOG=debug
RUST_BACKTRACE=1
SERVER_PORT=8080
SERVER_HOST=0.0.0.0
```

## Development

Start the development server with auto-reload:

```bash
bacon
```
The application will be available at: http://localhost:8080

> anytime you make a change, press r in the terminal the app is running to reload the changes.

## Project Structure

```
src/
├── config.rs     # Configuration settings
├── lib.rs        # Library root
├── main.rs       # Application entry point
├── model.rs      # Data models
└── routes/       # Route handlers
    ├── about.rs
    ├── admin.rs
    ├── journals.rs
    └── ...
```

## Features

- Journal article submission
- Current and past issues
- Editorial board management
- Admin interface
