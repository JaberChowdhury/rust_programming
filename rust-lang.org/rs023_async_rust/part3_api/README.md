# Part 3: API Development

## 🎯 What this part covers
This part covers building a complete, production-ready REST API using Axum, SQLx, and Tokio. You will learn how to structure a large Rust application, handle authentication, interact with a database, and manage application state.

## 📦 Prerequisites for this part
- Rust toolchain
- PostgreSQL database
- `sqlx-cli` installed (`cargo install sqlx-cli`)

## 🏋️ Exercises
1. `my_api`: A complete Axum REST API with JWT authentication, Postgres integration, and proper error handling.

## ✅ Key concepts checklist
- [ ] Application state management with `axum::extract::State`
- [ ] Routing and middleware in Axum
- [ ] Database connection pooling with SQLx
- [ ] Writing and running migrations
- [ ] JWT authentication and custom extractors
- [ ] Unified error handling with `IntoResponse`
- [ ] Configuration management
