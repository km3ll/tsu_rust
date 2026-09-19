# axum_postgres_crud

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

**Get all tasks**

```bash
curl -X GET -H "Accept: application/json" \
http://localhost:3000/tasks | jq
```

**Create task**

```bash
curl -X POST -H "Content-Type: application/json" \
-d '{"name": "Read tokyo documentation", "priority": 1 }' \
http://localhost:3000/tasks | jq
```

**Update task**

```bash
curl -X PATCH -H "Content-Type: application/json" \
-d '{"name": "Read tokyo documentation", "priority": 1 }' \
http://localhost:3000/tasks/2 | jq
```


**Delete task**

```bash
curl -X DELETE -H "Accept: application/json" \
http://localhost:3000/tasks/1 | jq
```

## Containers

- PGAdmin: http://localhost:8888

## References

- [Build a CRUD REST API with Rust Axum](https://www.youtube.com/watch?v=NJsTgmayHZY)