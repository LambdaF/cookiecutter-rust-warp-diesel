# Cookiecutter Rust-Warp-Diesel API
### A Cookiecutter template for a quick Rust REST API
This repo provides a template for [Cookiecutter](https://github.com/cookiecutter/cookiecutter) that creates a Rust based REST API using [Warp](https://docs.rs/warp/latest/warp/) to provide the API and routes, and [Diesel](https://diesel.rs/) to provide Postgres database writes. The `GET`, `GET/:id`, `POST` and `DELETE` endpoints are provided by default in conjunction with basic error handling, logging and an initial database table.

## Prerequisites
The following are required prior to usage:
- [Cookiecutter](https://github.com/cookiecutter/cookiecutter)
- [Cargo](https://doc.rust-lang.org/cargo/)
- [Diesel CLI](https://crates.io/crates/diesel_cli)
- The post-hook script is a shell script that expects standard GNU Linux tools (Cat, AWK, etc.)

## Installation and Usage
Pull the repository, and then in the level above the folder:

```
cookiecutter <folder name>
```

The installation will ask you for the following:
- `project_name`: The name of the crate to be generated, used in the `Cargo.toml`
- `postgres_db_url`: The database URL to write tables to. Ensure this is set correctly and that it's either a new database (if you have create permissions) or a database you don't mind writing new tables to.
- `initial_db_table_name`: Name of the initial database table used to seed the database; used throughout the code also. Creates a table with an `id: BIGSERIAL` and `name: String` field.

After either using the defaults or inputting your own values, code is generated and then the post-hook script begins to run, this will:
- Run diesel-cli setup
- Write initial database `up.sql` and `down.sql` for the `initial_db_table_name`
- Run migrations
- Perform a cargo install

If you would prefer that this did not happen automatically, simple remove/disable the script `hooks/post_gen_project.sh`.

## Acknowledgements
Code was largely adapted from an excellent tutorial at https://sgibala.com/01-01-rust-api-with-warp-and-diesel/ and so contains some similarities.