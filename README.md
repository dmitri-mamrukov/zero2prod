# Zero To Production In Rust

It is my project based on the *"Zero To Production In Rust A hands-on introduction
to backend development in Rust"* book by Luca Palmieri.

## Functional Tests

To check the health check handler:

```bash
curl --verbose http://127.0.0.1:8000/health_check
```

## Development

To continously watch code by checking and testing it:

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
