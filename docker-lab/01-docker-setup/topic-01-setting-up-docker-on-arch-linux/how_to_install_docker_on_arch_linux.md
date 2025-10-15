# How setup Docker on Arch Linux
_______________________________________________________________________________
### Install the required packages

The Docker Engine
```sh
sudo pacman -S --needed docker
```
_______________________________________________________________________________

The `docker-compose` command.

This a command that makes it easy to start multiple docker containers 
by using a `docker-compose.yml` file.

```sh
sudo pacman -S --needed docker-compose
```
_______________________________________________________________________________

The `docker-buildx` command.

Think of this as an upgraded version of the standard `docker build` command.

It allows you to build Docker images for different 
CPU architectures (like ARM or x86) and includes extra features 
for creating faster, more efficient builds.

```sh
sudo pacman -S --needed docker-buildx 
```
_______________________________________________________________________________
Bonus: 

LazyDocker 

A Go-powered terminal UI for the `docker` engine and the `docker-compose` command.
```sh
sudo pacman -S --needed lazydocker
```
_______________________________________________________________________________
