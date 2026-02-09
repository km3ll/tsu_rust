# axum_web_server

## Commands

```bash
cargo clean
cargo build
cargo fmt
cargo run
carto test -- --nocapture

docker build -t axum_web_server:latest .
docker run --rm -d --name axum_web_server_app -p 3000:3000 axum_web_server:latest
docker stop axum_web_server_app
```

## API

```bash
curl -X GET -H "Accept: application/json" \
http://localhost:3000/health | jq
```

## References

- [Creating an Axum Web Server in Rust is easy!](https://www.youtube.com/watch?v=FDWKlJmHv6k)
