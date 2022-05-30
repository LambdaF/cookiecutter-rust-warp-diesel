#!/usr/bin/sh
DATABASE_URL={{cookiecutter.postgres_db_url}}
diesel setup
DIESEL_MIGRATION_FOLDER=$(diesel migration generate create_{{cookiecutter.initial_db_table_name}} | awk 'BEGIN{FS = "/"};{print $2; exit}')

cat <<EOT > migrations/$DIESEL_MIGRATION_FOLDER/up.sql
CREATE TABLE {{cookiecutter.initial_db_table_name}} (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
)
EOT

cat <<EOT > migrations/$DIESEL_MIGRATION_FOLDER/down.sql
DROP TABLE {{cookiecutter.initial_db_table_name}}
EOT
diesel migration run

cargo install --path .