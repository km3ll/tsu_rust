# askama_todos

## Content

```
Rust Axum Askama Tutorial
- u01 Initial setup, routing, navigation and serving static files
- u02 Logging and extracting form data
- u03 Form validation and setup PostgreSQL database connection
```

## API

- http://localhost:8000

## Commands

```bash
cargo clean
cargo build
cargo fmt
cargo run
cargo test -- --nocapture
cargo watch -x run

sqlx database create
sqlx migrate add create_users_table
```

## References

- [Rust Axum Askama Tutorial](https://www.youtube.com/playlist?list=PLo5Oa5DU0IYnXbSRNQrFrAW804drtEqeU)
- [GitHub: axum_askama_tutorial](https://github.com/watery-desert/axum_askama_tutorial)
- [Jinja template engine](https://jinja.palletsprojects.com/en/stable/)
- axum
  - [extract](https://docs.rs/axum/latest/axum/extract/index.html#common-extractors)
  - [response::IntoResponse](https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html#required-methods)
- crates
  - [sqlx-cli](https://crates.io/crates/sqlx-cli)
  - [thiserror](https://docs.rs/thiserror/2.0.17/thiserror/)
  
