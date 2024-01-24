# Axum web server template

The repository is a template for building axum based web server

- with `PostgreSQL` as database & `Diesel` as orm
- `async-graphql` for grahql specification

## Inspired by Jeremy Chone [course](https://github.com/rust10x/rust-web-app)

Authorization & context handling was inspired by Jeremy Chone [youtube course](https://www.youtube.com/watch?v=3cA_mk4vdWY&t=6344s)

## To configure correctly this places should be updated:

- [] `.cargo/config.toml` -> databse url (localhost in example), pwd key & token key (replace `-` with `_` for RUST_LOG project name)
- [] `Cargo.toml` -> project name
- [] `docker-compose.yaml` -> at least db_name
- [] `.env` -> db url for diesel
- [] `DockerfileApp` -> `axum_diesel_async_graphql_template` should be replaced with project name (`-` replaced with `_`)
