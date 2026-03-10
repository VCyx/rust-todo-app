# Todo App

A simple full-stack Todo application built with Rust.

## Architecture

- **Backend**: Rust, Axum, SeaORM, PostgreSQL.
- **Frontend**: Rust, Leptos, Tailwind CSS (via Trunk).

## Getting Started

1. Start the database:

   ```bash
   docker-compose up -d
   ```

2. Run the backend server:

   ```bash
   cd server
   cargo run
   ```

3. Run the frontend UI:
   Ensure you have `trunk` and the `wasm32-unknown-unknown` target installed:
   ```bash
   cargo install trunk
   rustup target add wasm32-unknown-unknown
   ```
   Then start the development server:
   ```bash
   cd ui
   trunk serve
   ```

## Production Build

To build the application for a production release with all WASM size optimizations applied:

1. Build the backend:

   ```bash
   cd server
   cargo build --release
   ```

   _The optimized backend binary will be at `target/release/server`_

2. Build the frontend:
   ```bash
   cd ui
   trunk build --release
   ```
   _The optimized frontend web assets (HTML, CSS, JS, and compressed WASM) will be generated inside `ui/dist`_

## Features

- View all Todos on the Home page.
- Create new Todos.
- Click a Todo to view its details, edit, or delete it.
- Modern aesthetics with glassmorphism and subtle animations.
