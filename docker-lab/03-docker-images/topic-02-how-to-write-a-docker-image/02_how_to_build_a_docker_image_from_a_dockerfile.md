### To build your Docker image run this command:

Navigate to the directory where your `Dockerfile` is and run this command:

```dockerfile
FROM alpine:3.22.2
CMD ["echo", "This container is using Alpine Linux version 3.22.2"]
```

Run this command:
```sh
docker buildx build -t dezlymacauley/apline-simple-echo:3.22.2 .
```

The `.` at the end means run this command in the current directory.

The `-t` means tag. This basically gives the Docker Image a name that makes
it easy to reference later.

NOTE: This is my custom commmand
```sh
docker_images
```

```sh
REPOSITORY                         TAG       IMAGE ID       CREATED      SIZE
dezlymacauley/apline-simple-echo   3.22.2    c60056415b93   7 days ago   8.32MB
```
________________________________________________________________________________
