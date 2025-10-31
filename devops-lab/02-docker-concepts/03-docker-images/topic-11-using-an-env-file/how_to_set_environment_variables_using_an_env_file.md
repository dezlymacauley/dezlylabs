### Please Note:

.env
```dotenv
ENV PORT=9000
ENV APP_NAME="Goku"
```


```sh
docker run -d \
--env-file ".env" \
-p 127.0.0.1:9000:9000 \
--name expressjs-app-with-env-variables-image-instance-01 \
dezlymacauley/expressjs-app-with-env-variables-image:0.0.1
```
_______________________________________________________________________________
