# Zero To Production In Rust

It is my project based on the *"Zero To Production In Rust A hands-on introduction
to backend development in Rust"* book by Luca Palmieri.

## Functional Tests

To check the health check handler:

```bash
curl --verbose http://127.0.0.1:8000/health_check
```

To check the subscription handler:

```bash
curl --verbose --include --request POST --data \
    'email=thomas_mann@hotmail.com&name=Tom' http://127.0.0.1:8000/subscriptions
```

## Development

### Lint

```bash
cd app
./scripts/lint.sh
```

### Scripts

To create a Docker Postgres container and then to create and populate its DB:

```bash
cd app
scripts/init_db.sh
```

To create and populate the Postgres DB, skipping the Docker container creation:

```bash
cd app
SKIP_DOCKER=true scripts/init_db.sh
```

To stop and remove the Docker Postgres container named `zero2prod-postgres`:

```bash
docker container stop zero2prod-postgres
docker container rm zero2prod-postgres
```

### Postgres

Connect to Postgres. The password is in `scripts/init_db.sh`.

```bash
psql --host "localhost" --username "postgres" --port 5432
```

Show tables.

```bash
postgres=# \c newsletter
newsletter=# \dn
newsletter=# \dt
newsletter=# SELECT * FROM subscriptions;
newsletter=# SELECT * FROM _sqlx_migrations;
```

### Tests

To continously watch code by formatting, checking, linting, and testing it:

```bash
cd app
./scripts/watch.sh
```

To test with getting detailed and prettified output:

```bash
cd app
TEST_LOG=true cargo test health_check | bunyan
```

### Test coverage

To measure test coverage:

```bash
cd app
cargo tarpaulin --ignore-tests
```

### slqx

'prepare' performs the same work that is usually done when cargo build is
invoked but it saves the outcome of those queries to a metadata file
(.slx/query-<id>.json) which can later be detected by sqlx itself and used to skip the
queries altogether and perform an offline build.

```bash
cargo sqlx prepare -- --lib
```

We ensure that sqlx-data.json does not go out of sync (e.g. when the schema of
our database changes or when we add new queries).

```bash
cargo sqlx prepare --check -- --lib
```

### Docker

In one console:

```bash
docker build --tag zero2prod --file Dockerfile .
docker run --publish 8000:8000 zero2prod
```

In another console:

```bash
curl --verbose http://127.0.0.1:8000/health_check
curl --verbose --include --request POST --data \
    'email=thomas_mann@hotmail.com&name=Tom' http://127.0.0.1:8000/subscriptions
```
