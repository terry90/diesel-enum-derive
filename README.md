# Diesel enum derive

Please note that this crate is aimed to be simple and stick to a simple usage, no fancy configuration, only `Pg` with Text fields.

For a more advanced usage, see this crate: [adwhit/diesel-derive-enum](https://github.com/adwhit/diesel-derive-enum)

## Features

This crate has two features:

* `heck` uses `heck::ToSnakeCase` to transform enum values. This feature is enabled by default.
* `plain` keep enum values, use with `default-features = false`.

## Usage

```rust
#[derive(DieselEnum)]
pub enum Role {
    Admin,
    User,
}
```

The method `Role::Admin.db_value()` returns the database representation of this variant.

## Version compatibility

This crate is intended to be used with the [Diesel crate](https://crates.io/crates/diesel), but it's not a direct dependency of this crate.
Because of this reason, the version dependencies are not always clear, which can cause weird compilation errors in case of a mismatch.

The version compatibility is specified in the following table:

| Diesel version | Diesel-enum-derive version |
| -------------- | -------------------------- |
| <=1.4.8        | 0.1.4                      |
| \>=2.0.0       | 1.0.0                      |
