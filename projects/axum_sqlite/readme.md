# axum_sqlite

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

**Get index**

```bash
curl -X GET -H "Accept: text/html" \
http://localhost:3000
```

**Get all books**

```bash
curl -X GET -H "Accept: application/json" \
http://localhost:3000/books | jq
```

**Get book**

```bash
curl -X GET -H "Accept: application/json" \
http://localhost:3000/books/1 | jq
```

**Add book**

```bash
curl -X POST -H "Content-Type: application/json" \
-d '{ "id": 0, "title": "The Rust Programming Language", "author": "Steve Klabnik" }' \
http://localhost:3000/books/add | jq
```

**Edit book**

```bash
curl -X PUT -H "Content-Type: application/json" \
-d '{ "id": 2, "title": "Programming Rust", "author": "Blandy Jim" }' \
http://localhost:3000/books/edit | jq
```

**Delete book**

```bash
curl -X DELETE -H "Accept: application/json" \
http://localhost:3000/books/delete/1 | jq
```

## References

- [From Axum to CRUD Operations](https://www.youtube.com/live/JUWSy9pXgMQ)