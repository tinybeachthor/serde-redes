# serde-redes

> `redes` is just a random anagram of `serde`

This repo contains some arcane [serde](https://github.com/serde-rs/serde) uses.

The ultimate goal is to allow ergonomically extending `serde`:
- support some more advanced serialization (e.g. `Jinja` templated `toml`)
- allow easily extending the existing serializers with extra features
- add support for comments in `serde` output

## [serde-redes](./serde-redes/)



## [serde-ast](./serde-ast/)

[![Crates.io Version](https://img.shields.io/crates/v/serde-ast)](https://crates.io/crates/serde-ast)
[![docs.rs](https://img.shields.io/docsrs/serde-ast)](https://docs.rs/serde-ast/latest/serde_ast/)

Define an AST representation of `serde` serialization.

- [x] define an `Ast` for `serde` serialization
- [x] `Serializer` for `Ast`
- [x] `impl Serialize for Ast`
- [ ] `Deserializer` for `Ast`
- [ ] `impl Deserialize for Ast`

## ...

More coming soon(TM).
