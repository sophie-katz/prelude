# Contributing to Portobello

Portobello is intended to be developed inside VS Code. Everything that can be done inside VS Code can be done through other editors or via the command line, but these workflows are not supported.

* [Installing dependencies on your system](#installing-dependencies-on-your-system)
* [Opening the project in VS Code](#opening-the-project-in-vs-code)
* [Running a local instance for development](#running-a-local-instance-for-development)
    * [Port mapping](#port-mapping)
* [REST API client](#rest-api-client)
* [VS Code usage reference](#vs-code-usage-reference)
* [Known issues](#known-issues)

## Installing dependencies on your system

Development may be done on Windows, macOS, or Linux. The following dependencies need to be installed, however:

* [VS Code](https://code.visualstudio.com/)
* [Docker](https://www.docker.com/)
    * On Windows or macOS, install Docker Desktop
    * On Linux, install Docker server following [these instructions](https://docs.docker.com/engine/install/).
* [Docker Compose](https://docs.docker.com/compose/install/)
    * **NOTE:** Docker Desktop is most likely already installed with Docker. Use the command `docker-compose --version` to check (can be run on any supported platform).

## Opening the project in VS Code

Follow these steps to get your VS Code development environment up and running.

* **Clone:** Clone the source code from this repo.
* **Open:** Open this directory in VS Code.
* **Enter Dockerized environment:** Click the green button on the bottom left of the VS Code window, and then select "Reopen in Container". See [https://code.visualstudio.com/docs/remote/containers](https://code.visualstudio.com/docs/remote/containers) for more details.
    * It will take a while when done for the first time as it needs to download and build Docker images. Click "Show log" to see progress.
    * Once it completes, VS Code will be running inside a Dockerized development environment. Any code running inside VS Code is guaranteed to behave exactly the same as in production.
    * It will also start up third party services which will run alongside Portobello and allow it to run as in production.

Run `docker ps` to see the running containers.

## Running a local instance for development

* **Configure environment:** Copy `/app/.env.example.dev` to `/app/.env` and edit it, following the instructions in the comments within.
* **Install Yarn dependencies:** Install project dependencies from Yarn by openning a terminal in VS Code and running the command: `yarn`
* **Recreate databases:** To drop and recreate the databases, run the command: `yarn workspace @core/db run recreate-databases`
    * See [Known issues](#known-issues) if you run into an error here.
* **Migrate databases:** To drop and recreate the database schema, run the command: `yarn workspace @core/db run migrate-refresh`
    * If you run into issues, try recreating the databases and trying again.
* **Seed databases:** To seed the databases with the minimal data needed to run the application, run the command: `yarn workspace @utilities/db-seed run seed`
* **Configure Portobello admin user:** Go to http://localhost:9003/admin/master/console/#/portobello/users. This is the admin console for the local instance of Keycloak, an authentication service used by Portobello.
    * Click "Add user"
    * Enter in a username
    * Click "Create"
    * Navigate to the "Credentials" tab
    * Click "Set password"
    * Enter in a secure password
    * Disable the "Temporary" setting
    * Click "Save" and then "Save password"
    * You can now use these credentials to log into the local instance of Portobello.
* **Running server:** To run the server for the Portobello REST API, open a new terminal and run the command: `yarn workspace server run start`
    * This may take longer the first time as it has to install and build Cargo dependencies.
* **Running client:** To run the web client for Portobello, open a new terminal and run the command: `yarn workspace client run dev`
    * This may take longer the first time as it has to build the client.
    * Follow instructions on how to open the local instance in your browser.

See [`client/README.md`](/client/README.md) and [`server/README.md`](/server/README.md) for details on how to develop in the two main packages in this repo. Also see [Port map](PortMap.md) for a description of what ports Portobello uses for local development.

## REST API client

To set up the REST API in VS Code, import the Portobello collection in Thunder:
* Navigate to the Thunder sidebar panel
* Go to the Collections tab
* Using the hamburger menu, click Import
* Import `/app/portobello-thunder.json`

In order to update the REST API configuration, re-export over the same path.

## VS Code usage reference

The VS Code project is set up for the following features:

Category        | Functionality                           | Usage
--------------- | --------------------------------------- | -----
*Debugging*     | Debug an executable                     | Debug configurations exist for all relevant executables.<br /><br />**NOTE:** The project must be built first.
|               | Debug just one test                     | Just above each unit test in the code, there is a code lens with an option to debug the test.<br /><br />**NOTE:** Unfortunately, the Rust Test Adapter VS Code extension does not support debugging unit tests from the test explorer.<br /><br />**NOTE:** The project must be built first.
*Testing*       | Run all tests in project                | Tests are all available in the VS Code test explorer.<br /><br />**NOTE:** The project must be built first.
|               | Run tests in batch                      | Run the VS Code task: `yarn: test all workspaces`.
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
*Database*      | Run migrations                          | Run the VS Code task `yarn: update database migrations`.
|               | Generate entities                       | Run the VS Code task `yarn: generate database entities`.
|               | Seed database                           | Run the VS Code task `cargo: seed database`.

## Known issues

* If you run into errors saying that modules cannot be imported in `*.vue` files, you may need to set the typescript version. Run the VS Code command `Volar: Select Typescript Version...` and select the version prefixed with `.yarn/`.
* If you use "Reopen in Container" in VS Code and it errors out silently and immediately, try running the command "Dev Containers: Show Container Log" to get more details.
    * Frequently, this is caused by not copying `.env.example.dev` to `.env`.
