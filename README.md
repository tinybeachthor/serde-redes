# serde-redes

This repo contains some arcane [serde](https://github.com/serde-rs/serde) uses.

The ultimate goal is to allow ergonomically extending `serde`:
- support some more advanced serialization (e.g. `Jinja` templated `toml`)
- allow easily extending the existing serializers with extra features
- add support for comments in `serde` output

> `redes` is just a random anagram of `serde`

## [serde-ast](./serde-ast/)

Define an AST representation of `serde` serialization.

- [x] define an `Ast` for `serde` serialization
- [x] `Serializer` for `Ast`
- [x] `impl Serialize for Ast`
- [ ] `Deserializer` for `Ast`
- [ ] `impl Deserialize for Ast`

## ...

More coming soon(TM).
