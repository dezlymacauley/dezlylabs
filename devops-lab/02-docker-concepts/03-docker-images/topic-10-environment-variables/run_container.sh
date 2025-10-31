#!/usr/bin/env bash

docker run -d \
-p 127.0.0.1:5000:5000 \
--name expressjs-app-with-env-variables-image-instance-01 \
dezlymacauley/expressjs-app-with-env-variables-image:0.0.1
