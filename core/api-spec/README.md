# Portobello API specification

To generate the TypeScript/Rust bindings for OpenAPI, run:

```bash
yarn workspace @core/api-spec run generate
```

Or specific bindings can be generated individually:

```bash
# Rust binding
yarn workspace @core/api-spec run generate-client-rust

# TypeScript binding
yarn workspace @core/api-spec run generate-client-typescript-axios
```

To validate the specification without taking any other action:

```bash
yarn workspace @core/api-spec run validate
```
