#!/usr/bin/env bash

docker run -d \
-p 127.0.0.1:5000:5000 \
--name entrypoint-example-instance-01 \
dezlymacauley/entrypoint-example:0.0.1 \
"Clown World" 

# Unlike CMD, if you have `ENTRYPOINT` in your Dockerfile. 
# It will always print the entry point command, and then any additional cli
# command. You don't need to add echo

# hello from ENTRYPOINT in Dockerfile.entrypoint Clown World
