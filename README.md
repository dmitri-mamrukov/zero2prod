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
curl --verbose --include --request POST --data 'email=thomas_mann@hotmail.com&name=Tom' http://127.0.0.1:8000/subscriptions
```

## Development

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

To continously watch code by formatting, checking, and testing it:

```bash
cd app
cargo watch --exec fmt --exec check --exec test --exec run
```

### Test coverage

To measure test coverage:

```bash
cd app
cargo tarpaulin --ignore-tests
```
