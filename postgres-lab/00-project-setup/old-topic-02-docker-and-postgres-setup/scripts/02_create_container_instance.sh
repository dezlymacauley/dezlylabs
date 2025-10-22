#!/usr/bin/env bash

# If I run my custom shell command `docker_image`
# I get this output:

# REPOSITORY                    TAG       IMAGE ID       CREATED         SIZE
# dezlymacauley/postgres-dojo   0.0.1     562ad55a397c   2 minutes ago   280MB

# This is how to create a container instance from the image above.

docker run -d \
-p 127.0.0.1:5432:5432 \
--name postgres-dojo-instance-01 \
dezlymacauley/postgres-dojo:0.0.1
