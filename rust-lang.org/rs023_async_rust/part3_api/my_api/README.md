# My API

A complete REST API built with Axum, SQLx, and Tokio.

## Setup Instructions

1. Copy `.env.example` to `.env` and update the database URL.
2. Start a Postgres database.
3. Run migrations: `sqlx migrate run`
4. Run the server: `cargo run`

## Environment Variables

| Variable       | Description                      |
| -------------- | -------------------------------- |
| `DATABASE_URL` | Postgres connection string       |
| `JWT_SECRET`   | Secret key for signing JWTs      |
| `SERVER_PORT`  | Port to listen on (default 3000) |

## API Endpoints

| Method | Endpoint         | Description            | Auth Required |
| ------ | ---------------- | ---------------------- | ------------- |
| GET    | `/health`        | Health check           | No            |
| POST   | `/auth/register` | Register a new user    | No            |
| POST   | `/auth/login`    | Login and get JWT      | No            |
| GET    | `/users`         | List users (paginated) | Yes           |
| GET    | `/users/:id`     | Get user by ID         | Yes           |
| PUT    | `/users/:id`     | Update user            | Yes (Owner)   |
| DELETE | `/users/:id`     | Delete user            | Yes (Owner)   |

## Examples

Register:
`curl -X POST http://localhost:3000/auth/register -H "Content-Type: application/json" -d '{"username": "test", "password": "password"}'`
