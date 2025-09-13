# How to run a docker container
_______________________________________________________________________________
## What is a `docker image` and a `docker container`?

A docker image is a blueprint that contains everything you need to run
a specific application or program.

A docker container is an isolated environment that let's you run 
that application or program, 
without having to install it on your operating system.

_______________________________________________________________________________
### Start the docker service

```sh
sudo systemctl start docker
```
_______________________________________________________________________________
### How to check what docker images you have available

```sh
docker image ls
```
_______________________________________________________________________________
### How to check the `process status` of all containers

```sh
docker ps -a
```

You have to add `-a` to include all containers.

`docker ps` by default does not include containers that have been stopped.
_______________________________________________________________________________
### Go to the github repo of the program / application you want to download.

This is the official github repo of Nginx

https://github.com/nginx/nginx. 

Then put `/releases` after the name.

```
https://github.com/nginx/nginx/releases
```

E.g.

release-1.29.1

This is the latest stable version of Nginx at the time of this guide: 

`1.29.1`

_______________________________________________________________________________

### How to search a docker image

This is the official site for Docker Images.
```
https://hub.docker.com/
```

Go to the site and search for `Nginx`.

Look for `Trusted Content` and check `official Docker Image`

This is the official build of Nginx:
```
https://hub.docker.com/_/nginx
```
_______________________________________________________________________________
### How to download a Docker image

```sh
docker pull nginx:1.29.1
```

It should now be in your list of installed images:
```sh
docker image ls
```

```
docker image ls
REPOSITORY   TAG       IMAGE ID       CREATED       SIZE
nginx        1.29.1    41f689c20910   4 weeks ago   192MB
```

_______________________________________________________________________________

To run the container, open a separate tereminal and use this command:

Syntax is:

```
docker run -d \
--name what-you-want-to-call-the container \
the-image-to-use
```

E.g.
```sh
docker run -d \
--name nginx_container \
nginx:1.29.1
```

`-d` stands for detached mode. This will run the container as a background
process so that you can continue to use the terminal 
after entering the command.

_______________________________________________________________________________
