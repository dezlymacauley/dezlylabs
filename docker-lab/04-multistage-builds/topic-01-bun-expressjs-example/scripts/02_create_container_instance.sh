#!/usr/bin/env bash

# If I run my custom shell command `docker_image`
# I get this output:

# REPOSITORY                      TAG       IMAGE ID       CREATED          SIZE
# dezlymacauley/bun-express-app   0.0.1     4512384cd49d   26 minutes ago   119MB

# This is how to create a container from the image above.

docker run -d \
-p 127.0.0.1:3000:3000 \
--name bun-express-app-instance-01 \
dezlymacauley/bun-express-app:0.0.1

# To view the container log (The CMD line in the Dockerfile), 
# run this command:
# docker logs bun-express-app-instance-01
