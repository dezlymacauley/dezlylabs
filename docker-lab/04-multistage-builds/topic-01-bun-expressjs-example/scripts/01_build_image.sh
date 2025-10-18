#!/usr/bin/env bash

docker buildx build --load \
-t dezlymacauley/bun-express-app:0.0.1 \
-f ../Dockerfile ../

#______________________________________________________________________________

# ABOUT: `docker buildx build`

# This tells Docker to use the `build` command from the `buildx` plugin,
# instead of Docker's native build command.

# The `build` command from the buildx plugin can build Docker images 
# for multiple platform, and is faster than the regular build command. 

#______________________________________________________________________________

# ABOUT: `--load`

# This tells Docker to load the image into your local Docker image store.
# For testing and development use `--load`

# If you want to upload the image to Docker Hub or another registry,
# you just use `--push`.

# and --push when you want to upload the image to Docker Hub 
# or another registry.

#______________________________________________________________________________

# ABOUT: `-t dezlymacauley/cmd-example:0.0.1`

# This is the image tag. 
# This will be used to identify the image after it has been created.

# The syntax is:
# repository:version

# To be more specific:
# your-docker-hub-username/name-of-image/version (3 digits)

#______________________________________________________________________________

# ABOUT: `-f Dockerfile.cmd`

# This tells Docker which file contains the instructions to build the image.

#______________________________________________________________________________

# ABOUT: `../`

# The last part of the command is called the `build context` This tells
# Docker to use all the files and directories in the current directory
# to build the image.

# If you want certain files and directories to be ignored,
# they must be listed in a `.dockerignore` file.

#______________________________________________________________________________
