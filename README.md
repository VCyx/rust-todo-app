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

## Features

- View all Todos on the Home page.
- Create new Todos.
- Click a Todo to view its details, edit, or delete it.
- Modern aesthetics with glassmorphism and subtle animations.
