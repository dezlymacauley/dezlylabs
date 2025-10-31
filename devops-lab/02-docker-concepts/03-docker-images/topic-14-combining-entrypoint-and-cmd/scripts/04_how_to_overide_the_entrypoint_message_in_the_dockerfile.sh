#!/usr/bin/env bash

docker run -d \
-p 127.0.0.1:5000:5000 \
--name entrypoint-example-instance-01 \
--entrypoint echo \
dezlymacauley/entrypoint-example:0.0.1 \
"Clown World" 

# docker logs entrypoint-example-instance-01 
# Clown World
