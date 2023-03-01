# Portobello API server

A thin executable that launches the Portobello API server.

## Developer workflow

The server can be run with

```bash
yarn workspace server run start
```

This builds and runs the server, but does not support any hot loading.

## Running automated tests

The server should not need any automated tests of its own, but it depends on a number of Rust crates. The Rust code in this repo can be tested with:

```bash
cargo test
```

Alternatively, all of the code in this repo can be tested with:

```bash
yarn workspaces foreach run test
```
