# Full Stack Rust CRUD

A full-stack product CRUD application built with Rust, Axum, SQLx, PostgreSQL, and Yew.

This project is a personal interpretation of a Rust-first web application: an Axum backend exposes a JSON API backed by PostgreSQL, while the frontend is built in Rust with Yew and compiled to WebAssembly.

## Features

- Product CRUD API under `/api/products`
- PostgreSQL persistence through SQLx
- Embedded database migrations that run on server startup
- UUID product identifiers
- Product creation and update timestamps
- Axum middleware for tracing, CORS, request timeouts, and graceful shutdown
- Rust frontend with Yew, routing, forms, and API integration
- Single Rust-centered architecture for backend and frontend code

## Tech Stack

- Rust 2024
- Axum for HTTP routing
- Tokio for async runtime
- Tower and Tower HTTP for middleware
- SQLx for PostgreSQL queries and migrations
- PostgreSQL for persistence
- dotenv for local environment loading
- tracing for request/server logging
- Yew and WebAssembly for the frontend

## Project Layout

- `src/main.rs`: server startup, middleware, database initialization, and graceful shutdown
- `src/handlers/`: Axum route handlers
- `src/product.rs`: product model, service, and SQLx queries
- `src/db.rs`: PostgreSQL pool setup and embedded migration execution
- `src/config.rs`: environment-based configuration model
- `migrations/`: SQLx database migrations
- `bacon.toml`: local watch/task configuration

## Runtime Requirements

- Rust stable toolchain
- PostgreSQL database
- A `.env` file with a valid `DATABASE_URL`
- For the Yew frontend: Trunk and the `wasm32-unknown-unknown` Rust target

Example `.env`:

```env
DATABASE_URL=postgres://user:password@localhost:5432/full_stack_crud
```

The server loads `.env` on startup and expects `DATABASE_URL` to point to a reachable PostgreSQL database. SQLx migrations are embedded with `sqlx::migrate!()` and are applied automatically before the application starts serving requests.

## API

The backend listens on `127.0.0.1:3000`.

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/health` | Health check |
| `GET` | `/api/products/` | List products |
| `POST` | `/api/products/` | Create a product |
| `GET` | `/api/products/{id}` | Fetch one product by UUID |
| `PUT` | `/api/products/{id}` | Update a product by UUID |
| `DELETE` | `/api/products/{id}` | Delete a product by UUID |

Create a product:

```sh
curl -X POST http://127.0.0.1:3000/api/products/ \
  -H 'Content-Type: application/json' \
  -d '{"name":"Keyboard","price":120}'
```

List products:

```sh
curl http://127.0.0.1:3000/api/products/
```

## Database

The current schema is managed by SQLx migrations:

- Initial `products` table
- UUID primary key migration
- `created_at` and `updated_at` timestamp columns

The product API returns JSON shaped around the product record:

```json
{
  "id": "uuid",
  "name": "Keyboard",
  "price": 120,
  "created_at": "timestamp",
  "updated_at": "timestamp"
}
```

## Frontend

The frontend uses Yew components compiled to WebAssembly. It provides product listing, product creation forms, navigation, and API calls to the Axum backend.

The frontend includes:

- Product list page backed by `GET /api/products/`
- Product creation form backed by `POST /api/products/`
- Client-side routing for main product views
- Shared local development flow with the Axum API on port `3000`
