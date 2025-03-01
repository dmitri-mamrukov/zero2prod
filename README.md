# Zero To Production In Rust

It is my project based on the *"Zero To Production In Rust A hands-on
introduction to backend development in Rust"* book by Luca Palmieri.

## Application Development and Usage

### Prerequisites

#### slqx

Installation:

```bash
SQLX_VERSION=0.8.2
SQLX_FEATURES="rustls,postgres"
cargo install sqlx-cli \
    --version=$SQLX_VERSION \
    --features $SQLX_FEATURES \
    --no-default-features \
    --locked
```

'prepare' performs the same work that is usually done when cargo build is
invoked but it saves the outcome of those queries to a metadata file
(`.slx/query-<id>.json`) which can later be detected by sqlx itself and used to
skip the queries altogether and perform an offline build.

```bash
cd app
cargo sqlx prepare -- --lib
```

We ensure that `.slx/query-<id>.json` does not go out of sync (e.g. when the
schema of our database changes or when we add new queries).

```bash
cd app
cargo sqlx prepare --check -- --lib
```

### Option 1: Start from Scratch

1. There might be a Docker Postgres container named `local-zero2prod-postgres`
   from prior development. Check with

    ```bash
    docker ps --all
    ```

    ```bash
    docker container stop local-zero2prod-postgres
    docker container rm local-zero2prod-postgres
    ```

2. To create a Docker Postgres container and then to create and populate its DB:

    ```bash
    cd app
    scripts/init_db.sh
    ```

    If needed, to create and populate the Postgres DB, skipping the Docker
    container creation:

    ```bash
    cd app
    SKIP_DOCKER=true scripts/init_db.sh
    ```

3. Continue with step 2 in `Option 2: Continue with Existing Containers`.

### Option 2: Continue with Existing Containers

1. To start the Docker Postgres container named `local-zero2prod-postgres`:

    ```bash
    docker container start local-zero2prod-postgres
    ```

2. To lint your code:

    ```bash
    cd app
    ./scripts-dev/lint.sh
    ```

3. To continously watch code by formatting, checking, linting, and testing it:

    ```bash
    cd app
    ./scripts-dev/watch.sh
    ```

    Note: When upgrading the Rust compiler, you may get
    "Error: I/O error: Permission denied (os error 13)" errors, when you run
    `watch.sh`. In this case, do `cargo clean && cargo build` before running
    `watch.sh` again. If this doesn't help, then in the parent folder of
    `zero2prod` do:

    ```bash
    sudo chown --recursive <user> zero2prod
    ```

    where `<user>` is you.

4. To test with getting detailed and prettified output:

    ```bash
    cd app
    TEST_LOG=true cargo test health_check | bunyan
    ```

5. To measure test coverage:

    ```bash
    cd app
    cargo tarpaulin --ignore-tests
    ```

6. To build Docker containers:

    End the execution of `./scripts-dev/watch.sh` as it uses the application's
    port. Also shutdown the `local-zero2prod-postgres` container.

    ```bash
    docker container stop local-zero2prod-postgres
    ```

    In one console:

    ```bash
    cd app
    docker build --no-cache --tag zero2prod-app --file Dockerfile.app .
    docker build --no-cache --tag zero2prod-db-init --file Dockerfile.db-init .
    docker compose --file docker-compose.yml up --build
    ```

    Or

    ```bash
    cd app
    docker compose --file docker-compose.yml build --no-cache
    docker compose --file docker-compose.yml up --build
    ```

    In another console:

    ```bash
    cd app
    curl --verbose http://127.0.0.1:8000/health_check
    curl --verbose --include --request POST \
        --data 'email=thomas_mann@hotmail.com&name=Tom' \
        http://127.0.0.1:8000/subscriptions
    ```

    In another console:

    ```bash
    cd app
    docker compose --file docker-compose.yml down
    docker compose --file docker-compose.yml down --volumes
    ```

## Development

### Postgres

Connect to Postgres. The password is in `scripts/init_db.sh`.

```bash
psql --host "localhost" --username "postgres" --port 5432
```

Show schemas and tables in the `newsletter` database:

```bash
postgres=# \c newsletter
newsletter=# \dn
newsletter=# \dt
newsletter=# SELECT * FROM subscriptions;
newsletter=# SELECT * FROM _sqlx_migrations;
```

## Functional Tests

To check the health check handler:

```bash
curl --verbose http://127.0.0.1:8000/health_check
```

To check the subscription handler:

```bash
curl --verbose --include --request POST \
    --data 'email=thomas_mann@hotmail.com&name=Tom' \
    http://127.0.0.1:8000/subscriptions
```
