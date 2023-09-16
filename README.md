# zero2prod

It is my project based on the "Zero To Production In Rust A hands-on introduction to backend development in Rust" book by Luca Palmieri.

## Functional Tests

To check the health check handler:

```bash
curl --verbose http://127.0.0.1:8000/health_check
```

## Development

To continously watch code by checking and testing it:

```bash
cargo watch -x check -x clippy -x test -x run
```

### Test coverage

To measure test coverage:

```bash
cargo tarpaulin --verbose --workspace
```
