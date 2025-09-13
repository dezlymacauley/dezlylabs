
```sh
mkdir name-of-project
cd name-of-project
```

```sh
touch Dockerfile
```

Add this to the file:
```Dockerfile
FROM debian:bookworm
CMD ["echo", "Hello from Debian Bookworm" ]
```

To build your Docker image run this command:
```
docker buildx build .
```

Run this to see your image listed:
```sh
docker image ls
```
