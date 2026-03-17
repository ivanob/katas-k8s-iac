# Service A

Axum-based Rust microservice that exposes two endpoints:

- **GET /health** — Health check
- **POST /api/data** — Accept data payload

## Build

```bash
cargo build --release
```

## Run Locally

```bash
cargo run
```

Server will listen on `http://localhost:8080`

## Test

```bash
# Health check
curl http://localhost:8080/health

# Send data
curl -X POST http://localhost:8080/api/data \
  -H "Content-Type: application/json" \
  -d '{"message": "hello"}'
```

## Docker

```bash
docker build -t service-a:latest .
docker run -p 8080:8080 service-a:latest
```

## TODO

- [ ] Forward requests to service-b
- [ ] Add database integration
- [ ] Add error handling
- [ ] Add logging
