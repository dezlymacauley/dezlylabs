# Docker CLI Basics
_______________________________________________________________________________
### How to check if docker is active

This is the command
```sh
systemctl status docker.service
```

However I prefer to just create my own custom shell function and add
it to my .zshrc file.
```sh
docker_status() {
    if systemctl is-active --quiet docker.service; then
        echo "🟣 Docker is active"
    else
        echo "⚫ Docker is inactive"
    fi
}
```

Then I can just run:
```sh
docker_status
```
⚫ Docker is inactive
_______________________________________________________________________________
### How to start or stop the Docker Service and Docker Socket

`docker.socket` runs in the background, 
listens for docker commands that you enter using the docker cli, 
and then forwards these instructions to the docker service.

`docker.service` runs in the background and processes the commands 
it receives from the docker.socket.


To start:
```sh
sudo systemctl start docker.service docker.socket
```

To stop:
```sh
sudo systemctl stop docker.service docker.socket
```

My custom function.
```sh
docker_toggle() {
    if systemctl is-active --quiet docker.service; then
        # If Docker is active, deactivate it.
        sudo systemctl stop docker.service docker.socket
        echo "⚫ Docker has been deactivated"
    else
        # If Docker is inactive, activate it.
        sudo systemctl start docker.service docker.socket
        echo "🟣 Docker has been activated"
    fi
}
```
_______________________________________________________________________________
### How to login to Docker from the CLI

- First create a Docker Hub account.
- Go to your Profile -> Settings -> Personal access tokens
- Click `Generate New Token`

For this guide:
```
Access token description: docker-cli
Expiration date: 30 days
Access Permissions: Read, Write, Delete
```

- Click `Generate`

Save your token to a password manager.
_______________________________________________________________________________

[Docker Hub](https://hub.docker.com/)


docker login -u your-dockerhub-username

```sh
docker login -u dezlymacauley
```

Enter your personal access token: 

To log out:
```sh
docker logout
```
_______________________________________________________________________________
### How to check what docker images you have available

```sh
docker image ls
```

My custom command:
```sh
docker_images() {
    docker image ls
}
```

```
REPOSITORY                    TAG        IMAGE ID       CREATED         SIZE
redis                         alpine     d3f6d8be0b9b   11 days ago     76.6MB
gcr.io/k8s-minikube/kicbase   v0.0.48    c6b5532e987b   5 weeks ago     1.31GB
debian                        bookworm   a26cab9e734a   5 weeks ago     117MB
nginx                         1.29.1     41f689c20910   2 months ago    192MB
nginx                         1.27.0     900dca2a61f5   16 months ago   188MB
postgres                      14.2       9dbc24674f25   3 years ago     376MB
mailhog/mailhog               latest     4de68494cd0d   5 years ago     392MB
```
_______________________________________________________________________________

To remove a specific Docker image, target the image id
```sh
docker rmi  d3f6d8be0b9b
```

To remove all Docker images
```sh
docker rmi $(docker images -q)
```
_______________________________________________________________________________
### How to download a Docker image

Always make sure to include the version tag. E.g. Alpine:3.22.
```sh
docker pull alpine:3.22.2
```
_______________________________________________________________________________

### How to check the `process status` of all containers

```sh
docker ps -a
```

You have to add `-a` to include all containers.

`docker ps` by default does not include containers that have been stopped.

My custom commands. 
```sh
docker_containers_status() {
    docker ps -a --format "table {{.ID}}\t{{.Names}}\t{{.Status}}"
}

docker_containers_ports() {
    docker ps -a --format "table {{.ID}}\t{{.Names}}\t{{.Ports}}"
}
```

The first shows the container ID, Name, and Status.

The second shows the container ID, Name, and Port.

I find this mucher easier to read.

_______________________________________________________________________________
### How to stop containers

First you need the container status and its name or id
```sh
docker_containers_status
```

```
CONTAINER ID   NAMES                             STATUS
27aa39602918   subscription-service-redis-1      Up About an hour
f524a888413b   subscription-service-postgres-1   Up About an hour
db9ad336536e   subscription-service-mailhog-1    Up About an hour
03a3a9ad69ee   minikube                          Exited (137) 12 days ago
```

_______________________________________________________________________________
To stop a specific container, you can use its `container name`, 
or `container id`.

```sh
docker stop subscription-service-redis-1  
```
_______________________________________________________________________________

How to stop all containers
```sh
docker stop $(docker ps -q)
```

My custom command:
```sh
docker_stop_all_containers() {
    docker stop $(docker ps -q)
}
```
_______________________________________________________________________________
### How to remove a container

Note: First make sure that the containers that you are trying to remove
have been shut down.

To remove a specific container:
```sh
docker rm subscription-service-mailhog-1
```

To remove all containers:
```sh
docker rm $(docker ps -aq)
```

My custom command
```sh
docker_remove_all_containers() {
    docker rm $(docker ps -aq)
}
```
_______________________________________________________________________________
