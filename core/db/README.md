# Portobello database layer

This crate uses SeaORM to interact with the PostgreSQL database.

## Project setup

When starting off with a new PostgreSQL instance, all migrations must be run:

```bash
cd /app/core/db
sea-orm-cli migrate refresh
```

## Add new migration

```bash
cd /app/core/db
sea-orm-cli migrate generate <name>
```

## Generate entities

When changing the database structure, the entity code (Rust bindings for the database) must also be generated:

```bash
cd /app/core/db
sea-orm-cli generate entity -o src/entities
```

## Unit tests

All unit tests for the database are stored under `lib.rs`.
