# axum_web_server

## endpoints

- http://localhost:3000/health

### samples

```
curl -X GET -H "accept: application/json" http://localhost:3000/health | jq
```

## commands

```bash
cargo clean
cargo build
cargo fmt
cargo run
carto test -- --nocapture

docker build .
```

## references

- [Creating an Axum Web Server in Rust is easy!](https://www.youtube.com/watch?v=FDWKlJmHv6k)
