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

First create a Docker Hub account

[Docker Hub](https://hub.docker.com/)

```sh
docker login
```

You will need an account on Docker Hub for this to work.

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
