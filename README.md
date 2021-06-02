# rust-todos

A simple TO-DO list with a restful api written in Rust. Just because.

Rocket.rs for Web Application.
Diesel as ORM.
PostgreSQL for DB.

## Quickstart

### PostgreSQL

Run PG via Docker!

```
docker compose -f docker/docker-compose.yaml up -d
```

### Diesel / Migration

First things first. Install diesel cli

```
$ cargo install diesel_cli --no-default-features --features postgres
```

Setup the database

```
$ diesel setup
```

Apply migrations

```
diesel migration run
```