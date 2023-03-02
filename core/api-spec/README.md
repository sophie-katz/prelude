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
