#!/usr/bin/env bash

docker buildx build --load \
-t dezlymacauley/expressjs-app-with-env-variables-image:0.0.1 .
