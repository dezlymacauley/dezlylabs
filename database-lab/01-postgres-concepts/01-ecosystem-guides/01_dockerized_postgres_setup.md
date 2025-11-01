# Dockerized Postgres Setup
_______________________________________________________________________________
## Install Docker

_______________________________________________________________________________
### Create a custom pg_hba.conf file

Create this file locally
```sh
touch pg_hba.conf
```

Add this to the file
```
# TYPE  DATABASE        USER            ADDRESS                 METHOD
local   all             all                                     scram-sha-256
host    all             all             127.0.0.1/32            scram-sha-256
host    all             all             ::1/128                 scram-sha-256
```
_______________________________________________________________________________
## Download the latest Postgres image from Docker Hub

I will be using Postgres 18.0, with Trixie (branch of Debian) as 
a the base Linux image.

```sh
docker pull postgres:18.0-trixie
```
_______________________________________________________________________________

✅ Correct syntax pattern

docker run [OPTIONS] IMAGE [COMMAND] [ARG...]


Everything before the image name (here postgres:18.0-trixie) is an option,
and everything after it (if any) is a command that runs inside the container.
_______________________________________________________________________________

Create a container instance
```sh
docker run -d \
  --name postgres-secure \
  -e POSTGRES_PASSWORD=mysecretpassword \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_DB=postgres \
  -p 127.0.0.1:5432:5432 \
  -v /home/dezlymacauley/workspace/github-repos/dezlylabs/database-lab/01-postgres-concepts/02-project-templates/pg_hba.conf:/etc/postgresql/pg_hba.conf:ro \
  postgres:18.0-trixie \
  -c hba_file=/etc/postgresql/pg_hba.conf
```

The POSTGRES_PASSWORD is required. All the other environment variables are optional

This environment variable is required for you to use the PostgreSQL image. It must not be empty or undefined. This environment variable sets the superuser password for PostgreSQL. The default superuser is defined by the POSTGRES_USER environment variable.
_______________________________________________________________________________

Note everytime you pass in an environment variable, 
each variable must have the `-e` flag.

_______________________________________________________________________________

When you start a Postgres Docker container, the image automatically runs an initialization script that:

Creates a default database user named postgres

Creates a default database also named postgres

Sets the password for that user using the value you passed to the environment variable POSTGRES_PASSWORD
_______________________________________________________________________________

🔒 -p 127.0.0.1:5432:5432

This means:

“Expose port 5432 only on the host’s loopback interface (localhost).”

So:

The database can only be accessed from your own machine

It cannot be reached from other computers on the network

This is the safest setup for local development

_______________________________________________________________________________

  -e POSTGRES_HOST_AUTH_METHOD=scram-sha-256 \

_______________________________________________________________________________

To inspect the file system of the container using bash:
```sh
docker exec -it \
postgres-lab-instance-01 \
bash
```
_______________________________________________________________________________
Your prompt will change to something like this:

```sh
root@f166b72e57ee:/#
```

This means that you are logged in as a root Linux user,
and are interacting with the operating system (Debian - Trixie Branch), 
that is inside the container using Bash

The number after `root@` is actually the id of the Docker container.

You can confirm that you are using Bash shell by running the command:

```sh
echo $SHELL
```

To exit this shell just type:

```sh
exit
```
_______________________________________________________________________________

While using this shell prompt...
```sh
root@f166b72e57ee:/#
```

Run this to use `psql` to log into the default database called `postgres`, 
using the default user called `postgres`
```sh
psql \
-h 127.0.0.1 \
-p 5432 \
-d postgres \
-U postgres
```
_______________________________________________________________________________
### To view the connection info 

```sql
\conninfo
```
_______________________________________________________________________________
