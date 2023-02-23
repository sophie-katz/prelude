# Portobello

<img src="https://img.shields.io/badge/license-MIT-green" /> <img src="https://img.shields.io/badge/rust-1.69.0-blue" /> <img src="https://img.shields.io/badge/node-v18.13.0-blue" /> <img src="https://img.shields.io/badge/platform-linux%7Cdocker-lightgrey" />

Portobello is a project management and monitoring suite.

| Link | Description |
|------|-------------|
| [Google Drive folder](https://drive.google.com/drive/folders/1N00nt2MpcOYI9LJROfeZS94XxQnfkklY?usp=share_link) | Google Drive storage location |
| [REST API Documentation](https://portobello.stoplight.io/docs/portobello) | Documentation for REST API generated from OpenAPI spec with Spotlight |

## Project setup

* Click the green button on the bottom left of the VS Code window, and then select "Reopen in Container". See [https://code.visualstudio.com/docs/remote/containers](https://code.visualstudio.com/docs/remote/containers) for more details.
* Copy `.env.example.dev` to `.env` and edit it, following instructions within

This will install all dependencies needed within the container and reload the container. It will use a production-like Docker Compose environment and automatically launch all needed third-party services such as the database service.

**NOTE:** This will require [Docker](https://www.docker.com/) and [Docker Compose](https://docs.docker.com/compose/) to be installed on the host system and the [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension to be installed in VS Code.

## Running Portobello for development

These commands will launch the hot-loading development environment and allow you to develop in a production-like environment.

```bash
# Run database migrations (only before first run)
cd /app/core/db
sea-orm-cli migrate refresh

# Seed database (only before first run)
cd /app
cargo run --bin db-seed

# Launch server
cd /app
cargo run --bin server

# Launch client
yarn workspace client run dev
```

Before using Portobello, a user will have to be created. Go to http://localhost:8080/admin/master/console/#/portobello/users and log into Keycloak with the admin credentials in `.env` (`KEYCLOAK_ADMIN` and `KEYCLOAK_ADMIN_PASSWORD`).

* Click "Add user"
* Enter in a username
* Click "Create"
* Navigate to the "Credentials" tab
* Click "Set password"
* Enter in a secure password
* Disable the "Temporary" setting
* Click "Save" and then "Save password"

You may now log into Portobello as this user when prompted.

## Building and running production Portobello image

```bash
# All commands should be run from /app

# Run production Docker-compose
# TODO!
```

## Using VS Code to develop

The VS Code project is set up for the following features:

Category        | Functionality                           | Usage
--------------- | --------------------------------------- | -----
*Debugging*     | Debug an executable                     | Debug configurations exist for all relevant executables.<br /><br />**NOTE:** The project must be built first.
|               | Debug all tests for a Rust package      | Debug configurations exist for all relevant test binaries.<br /><br />**NOTE:** The project must be built first.
|               | Debug just one test                     | Just above each unit test in the code, there is a code lens with an option to debug the test.<br /><br />**NOTE:** Unfortunately, the Rust Test Adapter VS Code extension does not support debugging unit tests from the test explorer.<br /><br />**NOTE:** The project must be built first.
*Testing*       | Run all tests in project                | Tests are all available in the VS Code test explorer.<br /><br />**NOTE:** The project must be built first.
|               | Run tests in batch                      | Run the VS Code task: `rust: cargo test`.
*Linting*       | Lint the current file                   | Linting is automatically run after every file save.
*Formatting*    | Format the current file                 | Formatting is automatically done after every file save.
*Documentation* | Generate Rust documentation             | Run the VS Code task `rust: cargo doc` to generate HTML documentation for the whole project.
|               | View Rust documentation                 | Run the VS Code command `Rust: Doc Viewer`.<br /><br />**NOTE:** The project's HTML documentation must be generated first (see previous).
*Coverage*      | Generate inline Rust test coverage data | Run the VS Code task `rust: llvm-cov --lcov`. This will generate a `lcov.info` file which can be used to display inline test coverage.
|               | View inline Rust test coverage          | Run the VS Code command `Coverage Gutters: Display Coverage`.<br /><br />**NOTE:** The project's inline test coverage data must be generated first (see previous).
|               | Generate HTML Rust test coverage report | Run the VS Code task `rust: llvm-cov --html`.
|               | View HTML Rust test coverage report     | Run the VS Code command `Coverage Gutters: Preview Coverage Report`.<br /><br />**NOTE:** The project's HTML test coverage report must be generated first (see previous).
*API*           | Validate OpenAPI specification          | Run the VS Code task `yarn: validate api spec`.
|               | Generate OpenAPI bindings               | Run the VS Code task `yarn: generate api bindings`.
*Database*      | Run migrations                          | Run the VS Code task `db: migrate`.
|               | Generate entities                       | Run the VS Code task `db: generate entities`.
|               | Seed database                           | Run the VS Code task `db: seed`.

### REST API client

To set up the REST API in VS Code, import the Portobello collection in Thunder:
* Navigate to the Thunder sidebar panel
* Go to the Collections tab
* Using the hamburger menu, click Import
* Import `/app/portobello-thunder.json`

In order to update the REST API configuration, re-export over the same path.

### Database client

Portobello includes pgAdmin4 in its development environment. As long as the VS Code dev container is running, go to http://localhost:5050 to log into pgAdmin4. Use the login credentials provided in `.env`.

### Known issues

* If you run into errors saying that modules cannot be imported in `*.vue` files, you may need to set the typescript version. Run the VS Code command `Volar: Select Typescript Version...` and select the version prefixed with `.yarn/`.
* If you use "Reopen in Container" in VS Code and it errors out silently and immediately, try running the command "Dev Containers: Show Container Log" to get more details.
    * Frequently, this is caused by not copying `.env.example.dev` to `.env`.

## Project layout

| Path     | Description                                                         |
|----------|---------------------------------------------------------------------|
| `client` | Web frontend client subproject                                      |
| `core`   | Directory containing any packages/crates shared between subprojects |
| `docker` | Files relevant to running Portobello within Docker                  |
| `server` | Backend server subproject                                           |
