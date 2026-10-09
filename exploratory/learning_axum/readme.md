# learning_axum

[Brooks Builds - Learning Axum](https://www.youtube.com/playlist?list=PLrmY5pVcnuE-_CP7XZ_44HN-mDrLQV4nS)

## commands

```bash
cargo add tokio -F macros -F rt-multi-thread

# Location of the DB
# By default will be read from the DATABASE_URL env var or `.env` files]
sqlx database create
sqlx migrate add init
sqlx migrate info
sqlx migrate run

# Log syntax
RUST_LOG=default_level,module_path=level
```

## endpoints

- home: http://localhost:3000
- pgadmin: http://localhost:8888
- postgres: postgres:5432

## content

```
- Introduction to Axum
- Introduction to Axum 0.6
- Why Axum and Rust?
- Setting up VS Code
[ºº]
- Hello World!
```

## references

- [GitHub: project_solution](https://github.com/brooks-builds/full-stack-todo-rust-course/tree/main/backend/rust/axum/project_solution)
- [GitHub: tokio-rs/axum](https://github.com/tokio-rs/axum)