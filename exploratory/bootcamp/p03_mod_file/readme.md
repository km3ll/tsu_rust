# p03_mod_file

## commands

```
cargo-modules modules structure

crate p03_mod_file
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