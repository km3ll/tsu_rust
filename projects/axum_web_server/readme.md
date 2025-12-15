# axum_web_server

## Commands

```bash
cargo clean
cargo build
cargo fmt
cargo run
carto test -- --nocapture

docker build .
```

## API

**Get health check**

```bash
curl -X GET -H "Accept: application/json" \
http://localhost:3000/health | jq
```

## References

- [Creating an Axum Web Server in Rust is easy!](https://www.youtube.com/watch?v=FDWKlJmHv6k)
