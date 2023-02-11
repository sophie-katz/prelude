Add migration:

```bash
cd /app/core/db

sea-orm-cli migrate generate <name>
```

Run migrations:

```bash
cd /app/core/db

sea-orm-cli migrate refresh
```

Generate entities:

```bash
cd /app/core/db

sea-orm-cli generate entity -o src/entities
```
