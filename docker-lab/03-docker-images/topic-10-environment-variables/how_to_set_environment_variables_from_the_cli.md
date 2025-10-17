### Please Note:

1. You need to pass the `--env` for each environment variable
2. The environment variables that you set in the cli will overwrite,
the values set in the Dockerfile.

```sh
docker run -d \
--env PORT=4520 \
--env APP_NAME="Zeno" \
-p 127.0.0.1:4520:4520 \
--name expressjs-app-with-env-variables-image-instance-01 \
dezlymacauley/expressjs-app-with-env-variables-image:0.0.1
```
_______________________________________________________________________________
