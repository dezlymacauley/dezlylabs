How to run a simple container without creating a Docker file

This will automatically download the nginx version 1.27.0 image locally,
and then use it to create a Docker container called: `nginx-instance-001`

```sh
docker run -d --name nginx-instance-001 nginx:1.27.0
```

_______________________________________________________________________________
This is my custom shell command
```sh
docker_images
```

```
REPOSITORY   TAG       IMAGE ID       CREATED         SIZE
nginx        1.27.0    900dca2a61f5   16 months ago   188MB
```
_______________________________________________________________________________

This is my custom shell command

```sh
docker_containers_status
```

```
CONTAINER ID   NAMES                STATUS
114adcaf2c35   nginx-instance-001   Up 8 seconds
```

_______________________________________________________________________________

To enter the container using an interactive `bash` shell:

```sh
docker exec -it nginx-instance-001 bash
```

Your prompt will look like this:
```
root@ad48c3f5e53b:/#
```

To exit the bash shell type: `exit`

_______________________________________________________________________________

Note that some Docker images are extremely minimal 
so `bash` may not even be installed. 

So you can try to use `sh` instead.

```sh
docker exec -it nginx-instance-001 sh
```

Your prompt will look like this:
```
#
```

To exit the `sh` shell type: 
```
exit
```
_______________________________________________________________________________
