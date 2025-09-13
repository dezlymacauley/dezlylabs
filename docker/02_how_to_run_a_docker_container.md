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
### How to search for a docker image and its version

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

Scroll down to the section: `Supported tags and respective` for available
versions of the image you want.

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
-p 127.0.0.1:8080:80/tcp \
--name nginx_container \
nginx:1.29.1
```

`-d` stands for detached mode. This will run the container as a background
process so that you can continue to use the terminal 
after entering the command.

`-p` is short for publish.

`-p 127.0.0.1:8080:80/tcp` means I want to use `http://127.0.0.1` which is 
local on my computer (not exposed to to the internet), 
and I want `8080` on that network to be mapped 
to port `80` of the container `nginx_container`, over tcp.


NOTE: If you wanted to use `UDP` then you would use `/udp`.

_______________________________________________________________________________

To view the list of containers, run this command:
```sh
docker ps -a --format "table {{.Image}}\t{{.Status}}\t{{.Ports}}\t{{.Names}}"
```

You should see this:
```
IMAGE          STATUS         PORTS                    NAMES
nginx:1.29.1   Up 7 seconds   127.0.0.1:8080->80/tcp   nginx_container
```
_______________________________________________________________________________
### How to check that the container is working

I use the `httpie` cli to make a `GET` request.

```
http http://127.0.0.1:8080
```

You will get this back
```
<!DOCTYPE html>
<html>
<head>
<title>Welcome to nginx!</title>
<style>
html { color-scheme: light dark; }
body { width: 35em; margin: 0 auto;
font-family: Tahoma, Verdana, Arial, sans-serif; }
</style>
</head>
<body>
<h1>Welcome to nginx!</h1>
<p>If you see this page, the nginx web server is successfully installed and
working. Further configuration is required.</p>

<p>For online documentation and support please refer to
<a href="http://nginx.org/">nginx.org</a>.<br/>
Commercial support is available at
<a href="http://nginx.com/">nginx.com</a>.</p>

<p><em>Thank you for using nginx.</em></p>
</body>
</html>
```

You can also run this in the browser:
```
http://localhost:8080/
```
_______________________________________________________________________________
### To stop a container

Syntax is `docker stop name-of-container`

```sh
docker stop nginx_container
```

```
IMAGE          STATUS                     PORTS     NAMES
nginx:1.29.1   Exited (0) 8 seconds ago             nginx_container
```
_______________________________________________________________________________
### How to start a container again after stopping it

```
docker start nginx_container
```
_______________________________________________________________________________
### How to delete a container

```
docker rm nginx_container
```
_______________________________________________________________________________
### How to delete a docker image

```
docker image ls
```

REPOSITORY   TAG        IMAGE ID       CREATED       SIZE
debian       bookworm   a26cab9e734a   5 days ago    117MB
nginx        1.29.1     41f689c20910   4 weeks ago   192MB


Now you can use the `IMAGE ID` to delete it.

E.g. 
```
docker rmi a26cab9e734a
```

_______________________________________________________________________________
