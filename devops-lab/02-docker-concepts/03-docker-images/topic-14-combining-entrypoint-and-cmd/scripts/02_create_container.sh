#!/usr/bin/env bash

docker run -d \
-p 127.0.0.1:5000:5000 \
--name combining-entrypoint-and-cmd-instance-01 \
dezlymacauley/combining-entrypoint-and-cmd:0.0.1

# To view the container log (The CMD line in the Dockerfile), 
# run this command:
# docker logs combining-entrypoint-and-cmd-instance-01

# You will get this message if you did not supply any arguements
# Default Message
