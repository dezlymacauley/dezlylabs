#!/usr/bin/env bash

# This will create an instance from the container image
# dezlymacauley/bun-vite-react-typescript:0.0.1 

docker buildx build --load \
-t dezlymacauley/bun-vite-react-typescript:0.0.1 \
-f ../Dockerfile ../

docker run -d \
-p 127.0.0.1:3000:3000 \
--name bun-express-app-instance-01 \
dezlymacauley/bun-express-app:0.0.1

# To view the container log (The CMD line in the Dockerfile), 
# run this command:
# docker logs bun-express-app-instance-01
