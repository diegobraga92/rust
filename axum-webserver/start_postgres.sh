#!/bin/bash
# Start a PostgreSQL container with default credentials and database

# Load environment variables from .env
set -a
source .env
set +a

# Run the container (will create if not exists, or start if stopped)
docker run --name $CONTAINER_NAME \
  -e POSTGRES_PASSWORD=$POSTGRES_PASSWORD \
  -e POSTGRES_USER=$POSTGRES_USER \
  -e POSTGRES_DB=$POSTGRES_DB \
  -p $POSTGRES_PORT:5432 \
  -d postgres

echo "Postgres is running in Docker!"
echo "Connection string: postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@localhost:$PORT/$POSTGRES_DB"
echo "Connection string: postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@localhost:$POSTGRES_PORT/$POSTGRES_DB"
