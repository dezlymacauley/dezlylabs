#!/usr/bin/env bash

# This will create an instance from the container image
# dezlymacauley/bun-vite-react-typescript:0.0.1 

docker run -d \
-p 127.0.0.1:9000:80 \
--name bun-vite-react-typescript-instance-01 \
dezlymacauley/bun-vite-react-typescript:0.0.1
