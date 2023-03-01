# Portobello API specification

## Generating bindings

To generate the TypeScript bindings for the OpenAPI specification, run:

```bash
yarn workspace @core/api-spec run generate
```

This should be run

## Validating bindings

To validate the specification without taking any other action:

```bash
yarn workspace @core/api-spec run validate
```

## Developer workflow

If any changes are made to [`openapi.yml`](openapi.yml), they should also be made to the [`server-routes`](/core/server-routes) crate and to the [Thunder](https://www.thunderclient.com/) collection stored in [`portobello-thunder.json`](/portobello-thunder.json) (see [REST API client](/docs/Contributing.md#rest-api-client) in [Contributing](/docs/Contributing.md) for details).
