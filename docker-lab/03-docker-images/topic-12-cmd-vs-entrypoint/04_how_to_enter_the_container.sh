#!/usr/bin/env bash

# If I run my custom shell command `docker_image`
# I get this output:

# REPOSITORY                  TAG       IMAGE ID       CREATED      SIZE
# dezlymacauley/cmd-example   0.0.1     c6dcb143a3af   9 days ago   8.32MB

# This is how to create a container from the image above.

# `-it` interactive terminal
docker run -it \
-p 127.0.0.1:5000:5000 \
--name cmd-example-instance-01 \
dezlymacauley/cmd-example:0.0.1 \
sh

# The `sh` is the `sh` program
