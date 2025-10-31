#!/usr/bin/env bash

# `--rm` will remove the container as soon as ??
# `-it` This will enter the container using an interactive shell
# The shell will be sh. 

# `sh` is more reliable because some Docker images may not have bash
# as part of the base image.

# NOTE: Entering the interactive shell will 
# overwrite this line in the Dockerfile: CMD [ "node", "index.js" ]

docker run --rm -it dezlymacauley/hello-from-node:0.0.1 sh
