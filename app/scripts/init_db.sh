#!/usr/bin/env bash

set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed."
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    exit 1
fi

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${POSTGRES_HOST:=localhost}"

if [[ -z "${SKIP_DOCKER}" ]]
then
    docker run \
        --env POSTGRES_USER=${DB_USER} \
        --env POSTGRES_PASSWORD=${DB_PASSWORD} \
        --env POSTGRES_DB=${DB_NAME} \
        --publish "${DB_PORT}":5432 \
        --detach \
        --name local-zero2prod-postgres \
        postgres -N 1000
fi

export PGPASSWORD="${DB_PASSWORD}"
until psql --host "${DB_HOST}" --port "${DB_PORT}" --username "${DB_USER}" --dbname "postgres" --command '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
