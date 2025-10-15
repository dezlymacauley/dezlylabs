## Making comments

Use `#` to make comments
________________________________________________________________________________
## FROM

This is used to set the base image. 

E.g. A minimal install of an operating system.

```sh
FROM ubuntu:25.10
docker pull alpine:3.22.2
```
________________________________________________________________________________
## CMD

This is how you run commands.

```sh
CMD ["echo", "This container is using Ubuntu version 25.10"]
```
________________________________________________________________________________


To build your Docker image run this command:
```
docker buildx build .
```
________________________________________________________________________________
