FROM postgres:18.0-alpine3.22 AS build

WORKDIR /postgres-dojo/

# WARNING: Don't do this for a production application
ENV POSTGRES_DB=database_one
ENV POSTGRES_USER=user_one
ENV POSTGRES_PASSWORD=password_one
