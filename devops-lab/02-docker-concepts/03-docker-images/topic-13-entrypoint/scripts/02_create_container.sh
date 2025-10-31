#!/usr/bin/env bash

# If I run my custom shell command `docker_image`
# I get this output:

# REPOSITORY                         TAG       IMAGE ID       CREATED      SIZE
# dezlymacauley/entrypoint-example   0.0.1     73597fdbce12   9 days ago   8.32MB

# This is how to create a container from the image above.

docker run -d \
-p 127.0.0.1:5000:5000 \
--name entrypoint-example-instance-01 \
dezlymacauley/entrypoint-example:0.0.1

# To view the container log (The CMD line in the Dockerfile), 
# run this command:
# docker logs entrypoint-example-instance-01
