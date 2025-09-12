# bootcamp_p02

## commands

```
cargo-modules modules structure

crate bootcamp_p02
├── mod auth_utils: pub(crate)
│   ├── fn login: pub
│   ├── fn logout: pub
│   └── mod models: pub
│       └── struct Credentials: pub
├── fn authenticate: pub
└── mod database: pub(crate)
    ├── enum Status: pub
    ├── fn connect_to_db: pub
    └── fn get_user: pub
```