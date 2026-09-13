# MADS Usage Examples

This repository contains two HTTP application examples built with [MADS.rs](https://crates.io/crates/mads). Both examples include `user` and `post` modules, but they differ in how dependency injection (DI) is managed:

| Example | Database ORM | DI approach | Port |
| --- | --- | --- | --- |
| [`example_full_mads`](example_full_mads) | Diesel through MADS's database feature | MADS provides and assembles dependencies through the provider graph | `3000` |
| [`example_full_control`](example_full_control) | SeaORM | The user defines the database provider and chooses which dependencies are supplied to each component | `3003` |

## Application pattern

Both examples use the TPRS pattern: **Trait, Provider, Repository, Service**.

- **Trait** defines an application contract, such as `UserRepository`.
- **Provider** connects concrete implementations to contracts or other dependencies.
- **Repository** handles database operations.
- **Service** implements use cases and depends on repository traits.
- **Controller** receives HTTP requests and delegates work to services.

This structure keeps database query details out of controllers and services.

## Prerequisites

- Rust 1.85 or later
- PostgreSQL
- MADS CLI 0.8.0, for running `mads run` and `mads routes`

Create a `.env` file inside the example directory you want to run:

```dotenv
DATABASE_URL='postgres://username:password@localhost:5432/mads_test'
```

The URL is quoted so special characters such as `$` in the password are preserved when the dotenv file is loaded.

> Do not commit `.env`. Use `.env.example` if you want to share a configuration template.

## Example 1: full MADS usage

Change into [`example_full_mads`](example_full_mads). This example uses Diesel and MADS's `database` feature. MADS creates the `Database`, discovers the required providers, and assembles the dependency graph through to the controllers.

```text
UserController
  └─ Arc<dyn UserService>
      └─ UserServiceImpl
          └─ Arc<dyn UserRepository>
              └─ UserRepositoryImpl
                  └─ Database (Diesel/MADS)
```

In this example, MADS constructs `UserRepositoryImpl` and injects it into the user service. The post repository uses an explicit provider that accepts `Database`, while MADS still manages the dependency resolution and component lifecycle.

```bash
cd example_full_mads

# Apply the included migration
mads db migrate

# Start the API
mads run
```

Other available commands:

```bash
mads db status
mads db generate
mads routes
```

Use this example when you want to follow MADS conventions end-to-end and minimize database and dependency-graph setup boilerplate.

## Example 2: full control over dependency injection

Change into [`example_full_control`](example_full_control). This example uses SeaORM and does not enable MADS's Diesel database feature or migration commands.

The application defines its own `DatabaseModule`. Its `database_connection` provider reads `database.url` from the configuration, creates a `sea_orm::DatabaseConnection`, and exposes that type to the other modules.

```text
Config (database.url)
  └─ database_connection()
      └─ DatabaseConnection (SeaORM)
          ├─ UserRepositoryImpl → Arc<dyn UserRepository>
          │   └─ UserServiceImpl → Arc<dyn UserService>
          └─ PostRepositoryImpl → Arc<dyn PostRepository>
              └─ PostServiceImpl → Arc<dyn PostService>
```

`UserModule` and `PostModule` import `DatabaseModule`. Each repository provider explicitly accepts `DatabaseConnection`, allowing the user to choose the database implementation, connection setup, and dependencies made available to each module.

Apply the included SQL migration to the database, then start the API:

```bash
cd example_full_control

psql "$DATABASE_URL" -f migrations/01789196348572466463_schema_diff/up.sql

# Start the API
mads run
```

To list the registered routes:

```bash
mads routes
```

Use this example when you need explicit control over the ORM, connection creation, configuration, or dependency implementations—for example, when adding a mock or a custom database adapter.

## API endpoints

Both examples expose the following endpoints. The URL prefix and payloads are the same; use the port for the example you are running.

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/api/users/:id` | Find a user by numeric ID |
| `GET` | `/api/users/by-email/:email` | Find a user by email |
| `POST` | `/api/users` | Create a user |
| `PUT` | `/api/users/:id` | Update a user's name |

Example request for `example_full_mads`:

```bash
curl -X POST http://127.0.0.1:3000/api/users \
  -H 'Content-Type: application/json' \
  -d '{
    "email": "alice@example.com",
    "name": "Alice"
  }'
```

For `example_full_control`, change the port to `3003`:

```bash
curl -X POST http://127.0.0.1:3003/api/users \
  -H 'Content-Type: application/json' \
  -d '{
    "email": "alice@example.com",
    "name": "Alice"
  }'
```

A successful response is returned as JSON:

```json
{
  "id": 1,
  "name": "Alice",
  "email": "alice@example.com"
}
```

## Choosing an example

- Choose `example_full_mads` for a concise setup that uses MADS's Diesel integration.
- Choose `example_full_control` when you want to define the database provider and dependency graph yourself.

Each example also has its own documentation: [`example_full_mads/README.md`](example_full_mads/README.md) and [`example_full_control/README.md`](example_full_control/README.md).
