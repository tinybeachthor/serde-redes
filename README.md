# serde-redes

This repo contains some arcane [serde](https://github.com/serde-rs/serde) uses.
The ultimate goal is to allow ergonomically extending `serde`,
to support some more advanced serialization (e.g. `Jinja` templated `toml`)
without rewriting the serialization libraries completely.

> `redes` is a randomly shuffled `serde`

## [serde-ast](./serde-ast/)

- [x] define an `Ast` for `serde` serialization
- [x] `Serializer` for `Ast`
- [x] `impl Serialize for Ast`
- [ ] `Deserializer` for `Ast`
- [ ] `impl Deserialize for Ast`
