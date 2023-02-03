# Portobello

<img src="https://img.shields.io/badge/license-MIT-green" /> <img src="https://img.shields.io/badge/rust-1.69.0-blue" /> <img src="https://img.shields.io/badge/node-v18.13.0-blue" /> <img src="https://img.shields.io/badge/platform-linux%7Cdocker-lightgrey" />

Portobello is a project management and monitoring suite.

| Link                                                                                                           | Description                   |
|----------------------------------------------------------------------------------------------------------------|-------------------------------|
| [Google Drive folder](https://drive.google.com/drive/folders/1N00nt2MpcOYI9LJROfeZS94XxQnfkklY?usp=share_link) | Google Drive storage location |

## Project setup

Click the green button on the bottom left of the VS Code window, and then select "Reopen in Container". See [https://code.visualstudio.com/docs/remote/containers](https://code.visualstudio.com/docs/remote/containers) for more details.

This will install all dependencies needed within the container and reload the container. It will use a production-like Docker Compose environment and automatically launch all needed third-party services such as the database service.

**NOTE:** This will require [Docker](https://www.docker.com/) and [Docker Compose](https://docs.docker.com/compose/) to be installed on the host system and the [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension to be installed in VS Code.

## Using VS Code to develop

The VS Code project is set up for the following features:

Category        | Functionality                           | Usage
--------------- | --------------------------------------- | -----
*Debugging*     | Debug an executable                     | Debug configurations exist for all relevant executables.<br /><br />**NOTE:** The project must be built first.
|               | Debug all tests for a Rust package      | Debug configurations exist for all relevant test binaries.<br /><br />**NOTE:** The project must be built first.
|               | Debug just one test                     | Just above each unit test in the code, there is a code lens with an option to debug the test.<br /><br />**NOTE:** Unfortunately, the Rust Test Adapter VS Code extension does not support debugging unit tests from the test explorer.<br /><br />**NOTE:** The project must be built first.
*Testing*       | Run all tests in project                | Tests are all available in the VS Code test explorer.<br /><br />**NOTE:** The project must be built first.
*Linting*       | Lint the current file                   | Linting is automatically run after every file save.
*Formatting*    | Format the current file                 | Formatting is automatically done after every file save.
*Documentation* | Generate Rust documentation             | Run the VS Code task `rust: cargo doc` to generate HTML documentation for the whole project.
|               | View Rust documentation                 | Run the VS Code command `Rust: Doc Viewer`.<br /><br />**NOTE:** The project's HTML documentation must be generated first (see previous).
*Coverage*      | Generate inline Rust test coverage data | Run the VS Code task `rust: llvm-cov --lcov`. This will generate a `lcov.info` file which can be used to display inline test coverage.
|               | View inline Rust test coverage          | Run the VS Code command `Coverage Gutters: Display Coverage`.<br /><br />**NOTE:** The project's inline test coverage data must be generated first (see previous).
|               | Generate HTML Rust test coverage report | Run the VS Code task `rust: llvm-cov --html`.
|               | View HTML Rust test coverage report     | Run the VS Code command `Coverage Gutters: Preview Coverage Report`.<br /><br />**NOTE:** The project's HTML test coverage report must be generated first (see previous).

## Running Portobello for development

These commands will launch the hot-loading development environment and allow you to develop in a production-like environment.

```bash
# All commands should be run from /app

# Launch server
# TODO!

# Launch client
# TODO!
```

## Building and running production Portobello image

```bash
# All commands should be run from /app

# Run production Docker-compose
# TODO!
```

## Project layout

| Path     | Description                                                         |
|----------|---------------------------------------------------------------------|
| `client` | Web frontend client subproject                                      |
| `core`   | Directory containing any packages/crates shared between subprojects |
| `docker` | Files relevant to running Portobello within Docker                  |
| `server` | Backend server subproject                                           |
