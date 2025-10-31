First you need to be logined into to `Docker Hub`

```sh
docker login
```
_______________________________________________________________________________

```sh
docker_images
```

I'm starting off with this:
```
REPOSITORY                         TAG       IMAGE ID       CREATED      SIZE
dezlymacauley/apline-simple-echo   3.22.2    c60056415b93   7 days ago   8.32MB
```

Push the image to GitHub
```sh
docker push dezlymacauley/apline-simple-echo:3.22.2
```

Your image has been published:
```
https://hub.docker.com/r/dezlymacauley/apline-simple-echo
```
_______________________________________________________________________________
