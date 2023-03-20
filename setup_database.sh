#!/bin/bash

# Load the .env file and extract the PostgreSQL credentials
source <(grep '^PG\.' .env)

# Create the chathistory database
PGPASSWORD="${PG_PASSWORD}" createdb -U "${PG_USER}" -h "${PG_HOST}" -p "${PG_PORT}" "${PG_DBNAME}"

# Execute the create_tables.sql file to create the necessary tables
PGPASSWORD="${PG_PASSWORD}" psql -U "${PG_USER}" -h "${PG_HOST}" -p "${PG_PORT}" -d "${PG_DBNAME}" -f create_tables.sql
