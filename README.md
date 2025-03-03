# African Academic Union - African Journal of Educational Technology

A web application for managing and publishing academic journals.

## Prerequisites

- Rust (latest stable version)
- Cargo
- Git

## Setup

1. Clone the repository:
```bash
git clone https://github.com/yourusername/aau-ajet.git
cd aau-ajet
```

2. Install development dependencies:
```bash
cargo install cargo-watch
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
cargo watch -x 'run'
```
The application will be available at: http://localhost:8080

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
