# Service B

Spring Boot Java 21 microservice that tracks request counts.

## Endpoints

- **POST /api/data** — Accept a message, increment counter
  ```bash
  curl -X POST http://localhost:8080/api/data \
    -H "Content-Type: application/json" \
    -d '{"message": "hello"}'
  ```

- **GET /api/health** — Health check
  ```bash
  curl http://localhost:8080/api/health
  ```

## Build

```bash
mvn clean package
```

## Run Locally

```bash
mvn spring-boot:run
```

Server will listen on `http://localhost:8080`

## Docker

```bash
docker build -t service-b:latest .
docker run -p 8080:8080 service-b:latest
```

## TODO

- [ ] Store request count in PostgreSQL database
- [ ] Call service-a if needed
- [ ] Add error handling
