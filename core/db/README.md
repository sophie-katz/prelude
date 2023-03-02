<!--
MIT License

Copyright (c) 2023 Sophie Katz

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
-->

# Portobello database layer

This crate uses SeaORM to define and interact with the PostgreSQL database.

## Project setup

When starting off with a new PostgreSQL instance, the following commands can be used to set up the database:

```bash
yarn workspace @core/db run recreate-databases
yarn workspace @core/db run migrate-refresh
yarn workspace @utilities/db-seed run seed
```

See [Contributing](/docs/Contributing.md) for more details on project setup.

## Add new migration

To make a change to the database schema, use a [SeaORM migration](https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration/). A new one can be greated with:

```bash
cd /app/core/db
sea-orm-cli migrate generate <name>
```

Where `<name>` is the name of a migration. It should be a valid Rust identifier such as `my_cool_migration`.

## Generate entities

When changing the database structure, the [entity code](https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/) (Rust bindings for the database) must also be generated:

```bash
yarn workspace @core/db run generate-entities
```

**NOTE:** Entities are generated off of the `portobello_dev` database, so make sure that this is fully migrated before running this.
