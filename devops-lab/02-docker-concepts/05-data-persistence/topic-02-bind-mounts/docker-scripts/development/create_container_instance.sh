#!/usr/bin/env bash

# This will create an instance from the container image
# dezlymacauley/bun-vite-react-ts-dev-img:0.0.1 

# Get project root directory (two levels up from this script)
LOCAL_ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

# Working directory inside the container

# NOTE: The name should match with what is in the Dockerfile.dev 
WORKING_DIR_OF_CONTAINER="/app"

docker run -d \
-p 127.0.0.1:6969:6969 \
-v "$LOCAL_ROOT_DIR:$WORKING_DIR_OF_CONTAINER" \
--name bun-vite-react-ts-dev-img-instance-01 \
dezlymacauley/bun-vite-react-ts-dev-img:0.0.1

#______________________________________________________________________________

# NOTE: How to use bind mounts

# -v local-directory-path:/working-directory-of-container/
# In my Dockerfile: `WORKDIR /app/`

# ABOUT: What is the point of bind mounts:

# if your container runs a server watching /app/src and /app/public, 
# this setup will allow hot-reloading because you’re mounting 
# the host directories directly.

#______________________________________________________________________________
# BUG: How to Fix the "Hot reloading stops working issue"

# Add this line to:
# "dev": "CHOKIDAR_USEPOLLING=true CHOKIDAR_INTERVAL=100 vite --host 0.0.0.0 --port 6969",

# Vite watches files to reload the browser when they change.
#
# Normally it uses OS notifications to know when a file changes.

# Inside a Docker container, 
# these notifications sometimes don’t work correctly with mounted files.

# By adding CHOKIDAR_USEPOLLING=true, 
# you tell Vite to keep checking the files regularly instead 
# of waiting for notifications.

# This makes hot-reloading reliable every time, 
# no matter the container or OS.

# Basically: polling = “keep checking for changes”,
# which works inside Docker, while normal watching can fail.

#______________________________________________________________________________
