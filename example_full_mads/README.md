# MADS User API

## TPRS philosophy

This project follows the **TPRS (my made up name)** pattern: **Trait, Provider, Repository, and
Service**. The pattern separates application contracts from their concrete
implementations and lets MADS assemble dependencies through its provider
graph.

- **Trait** defines the capability required by the application. For example,
  `UserRepository` describes user persistence without exposing Diesel queries.
- **Provider** binds a concrete managed component to its trait object. In this
  module, `provider.rs` converts `UserRepositoryImpl` into
  `Arc<dyn UserRepository>` and registers it with MADS.
- **Repository** contains database-specific operations. `UserRepositoryImpl`
  owns Diesel queries and receives the managed `Database` dependency.
- **Service** coordinates use cases through trait contracts. `UserServiceImpl`
  depends on `Arc<dyn UserRepository>`, so its logic is independent of the
  selected persistence implementation.

The controller remains outside the TPRS layers. It is responsible for HTTP
input and output, then delegates business operations to `UserService`. This
structure makes persistence implementations replaceable and keeps database
queries out of the HTTP boundary.

This project is a Rust HTTP API built with MADS.rs, Diesel, and PostgreSQL. It
provides a user module with a layered dependency structure:

```text
UserController → Arc<dyn UserService> → UserServiceImpl → Arc<dyn UserRepository> → UserRepositoryImpl → Database
```

`UserRepositoryImpl` owns the Diesel queries. The module registers the concrete
repository and service as trait objects in `provider.rs`, which keeps the
service and controller independent of the persistence implementation.

## Prerequisites

- Rust 1.85 or later
- PostgreSQL
- MADS CLI 0.8.0

Create a `.env` file in the project root with the database connection URL:

```dotenv
DATABASE_URL='postgres://username:password@localhost:5432/mads_test'
```

The value is quoted so characters such as `$` in a password are preserved when
MADS loads the dotenv file.

## Database setup

Generate a migration after changing the Diesel schema, inspect the generated
SQL, then apply it:

```bash
mads db generate
mads db migrate
```

To inspect migration state:

```bash
mads db status
```

## Running the API

The default server address is `127.0.0.1:3000`.

```bash
mads run
```

List the registered routes with:

```bash
mads routes
```

## User endpoints

All user endpoints are prefixed with `/api`.

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/api/users/:id` | Find a user by numeric ID. |
| `GET` | `/api/users/by-email/:email` | Find a user by email address. |
| `POST` | `/api/users` | Create a user. |
| `PUT` | `/api/users/:id` | Update a user's name. |

### Create a user

```bash
curl -X POST http://127.0.0.1:3000/api/users \
  -H 'Content-Type: application/json' \
  -d '{
    "email": "alice@example.com",
    "name": "Alice"
  }'
```

The request body requires `email` and `name`. A successful request returns the
created user as JSON:

```json
{
  "id": 1,
  "name": "Alice",
  "email": "alice@example.com"
}
```

### Update a user name

```bash
curl -X PUT http://127.0.0.1:3000/api/users/1 \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "Alice Smith"
  }'
```
