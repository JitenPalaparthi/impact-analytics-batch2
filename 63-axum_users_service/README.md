# axum_users_service

Axum-based CRUD service for a `users` table in Postgres.

## Requirements

- Rust + Cargo
- Docker (optional, for Postgres)
- Postgres database

## Quick Start

### 1. Start Postgres with `users` table

Example using Docker:

```bash
docker run --name usersdb \
  -e POSTGRES_DB=usersdb \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=password \
  -p 5432:5432 \
  -v "$PWD/db/init.sql:/docker-entrypoint-initdb.d/init.sql:ro" \
  -d postgres:16
```

### 2. Configure environment

Create a `.env` file:

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/usersdb
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
```

### 3. Run the service

```bash
cargo run
```

### 4. Test the API

- Health:

```bash
curl http://localhost:3000/health
```

- Create user:

```bash
curl -X POST http://localhost:3000/users \
  -H 'Content-Type: application/json' \
  -d '{"name":"Jiten","email":"jiten@example.com","age":40}'
```

- List users:

```bash
curl http://localhost:3000/users
```

- Get user:

```bash
curl http://localhost:3000/users/1
```

- Update user:

```bash
curl -X PUT http://localhost:3000/users/1 \
  -H 'Content-Type: application/json' \
  -d '{"age": 41}'
```

- Delete user:

```bash
curl -X DELETE http://localhost:3000/users/1
```
