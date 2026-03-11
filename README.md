# Todo App

A Rust todo application built with Axum, SeaORM, PostgreSQL, Leptos, and Tailwind CSS.
The project now uses the standard Leptos fullstack SSR approach: Axum renders the app through `leptos_axum`, Leptos hydrates it in the browser, and data loading/mutations go through Leptos `#[server]` functions.

## Architecture

- Backend: Rust, Axum, SeaORM, PostgreSQL.
- Frontend: Rust, Leptos, Tailwind CSS.
- SSR integration: `leptos_axum`.
- Data flow: Leptos server functions, not a separate browser-side REST client.
- Static assets: built by Trunk into `ui/dist` and served by the Axum server.
- App server: [http://localhost:3000](http://localhost:3000).

## Requirements

- Rust toolchain
- `wasm32-unknown-unknown` target
- `trunk`
- Docker or a local PostgreSQL instance

Install missing frontend tooling if needed:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

## Local Development

Run the project with three processes.

1. Start PostgreSQL from the project root:

   ```bash
   docker-compose up -d
   ```

2. Start frontend asset rebuild in watch mode:

   ```bash
   cd ui
   trunk watch
   ```

3. In another terminal, start the SSR server:

   ```bash
   cd server
   cargo run
   ```

4. Open the application at [http://localhost:3000](http://localhost:3000).

### What `trunk watch` does

- `trunk watch` does not start the app server.
- It only rebuilds the browser assets in `ui/dist` when files in `ui/src` or `ui/style` change.
- The actual application is always served by the Axum server at [http://localhost:3000](http://localhost:3000).
- After a frontend change, refresh the page in the browser.

### Recommended Terminal Setup

Terminal 1:

```bash
docker-compose up -d
```

Terminal 2:

```bash
cd ui
trunk watch
```

Terminal 3:

```bash
cd server
cargo run
```

With this setup, Trunk continuously rebuilds `ui/dist`, and the Axum server on [http://localhost:3000](http://localhost:3000) serves both SSR pages and the current frontend assets.

## Production-Style Start

If you want to start the app without watch mode:

```bash
docker-compose up -d
cd ui
trunk build
cd ..\server
cargo run
```

Then open [http://localhost:3000](http://localhost:3000).

## Notes

- This project uses the standard Leptos SSR pipeline rather than a custom HTML shell.
- The `ui` crate is built as a wasm library and exports `hydrate()` for client hydration.
- Server-rendered routes and hydrated client navigation use the same Leptos router.
- If `ui/dist` is stale or missing, rebuild it with `trunk build` or keep `trunk watch` running during development.
- `trunk serve` is not the main entrypoint for this project.

## Features

- View all todos on the home page.
- Create new todos.
- Open a todo detail page, edit it, or delete it.
- Get server-rendered first paint with client-side hydration and navigation.
