# Portobello database layer

This crate uses SeaORM to interact with the PostgreSQL database.

## Project setup

When starting off with a new PostgreSQL instance, all migrations must be run:

```bash
yarn workspace @core/db run migrate-update
```

## Add new migration

```bash
cd /app/core/db
sea-orm-cli migrate generate <name>
```

## Generate entities

When changing the database structure, the entity code (Rust bindings for the database) must also be generated:

```bash
yarn workspace @core/db run generate-entities
```

**NOTE:** Entities are generated off of the `portobello_dev` database, so make sure that this is fully migrated before running this.

## Unit tests

All unit tests for the database are stored under `lib.rs`.
