# Diesel enum derive

Please note that this crate is aimed to be simple and stick to a simple usage, no fancy configuration, only `Pg` with Text fields.

For a more advanced usage, see this crate: [adwhit/diesel-derive-enum](https://github.com/adwhit/diesel-derive-enum)

## Usage

```rust
#[derive(DieselEnum)]
pub enum Role {
    Admin,
    User,
}
```

The method `Role::Admin.db_value()` returns the database representation of this variant.
