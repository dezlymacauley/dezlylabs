# Docker and Postgres Setup
_______________________________________________________________________________

_______________________________________________________________________________
## Build the `postgres-dojo` Docker Image

First run `docker_status` to check if the Docker service is active.

If it is inactive, then run `docker_toggle` to activate it.


Ensure that you are in the root of this project directory.

This is where the `Dockerfile` is located).

Then run these commands:
```sh
cd ./scripts/
bash 01_build_image.sh
```

To view Docker Image run this command:
```sh
docker_images
```

```
REPOSITORY                    TAG       IMAGE ID       CREATED          SIZE
dezlymacauley/postgres-dojo   0.0.1     562ad55a397c   29 minutes ago   280MB
```

Note: `docker_images` is my own custom shell function in my `.zshrc` file


_______________________________________________________________________________
## Create a container instance using the `postgres-dojo` Docker Image

Ensure that you are in the root of this project directory.

This is where the `Dockerfile` is located).

Then run these commands:

```sh
cd ./scripts/
bash 02_create_container_instance.sh
```
_______________________________________________________________________________

#!/usr/bin/env bash

docker exec -it postgres-dojo-instance-01 sh


psql -U user_one -d database_one

database_one=#


SELECT current_user;

 current_user
--------------
 user_one
(1 row)


\conninfo

           Connection Information
      Parameter       |        Value
----------------------+---------------------
 Database             | database_one
 Client User          | user_one
 Socket Directory     | /var/run/postgresql
 Server Port          | 5432
 Options              |
 Protocol Version     | 3.0
 Password Used        | false
 GSSAPI Authenticated | false
 Backend PID          | 82
 SSL Connection       | false
 Superuser            | on
 Hot Standby          | off
(12 rows)

_______________________________________________________________________________
