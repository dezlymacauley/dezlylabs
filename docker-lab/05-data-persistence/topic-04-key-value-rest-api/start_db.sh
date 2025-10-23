#!/usr/bin/env bash

MONGODB_IMAGE="mongodb/mongodb-community-server"
MONGODB_TAG="7.0-ubuntu2204"
CONTAINER_NAME="mongodb"

# Root credentials
ROOT_USER="root-user"
ROOT_PASSWORD="root-password"

# `--rm` whenever the container is stopped, it will automatically be removed.
# `-d` This will run the container as a detatched process. This simply
# means that it will run in the background so that you can still 
# use the terminal that you use to run this command

# `--name` is the name of the container
# `-e` is how you pass environment variables to the container  
docker run --rm -d \
--name $CONTAINER_NAME \
-e MONGODB_INITDB_ROOT_USERNAME=$ROOT_USER \
-e MONGODB_INITDB_ROOT_PASSWORD=$ROOT_PASSWORD \
$MONGODB_IMAGE:$MONGODB_TAG
