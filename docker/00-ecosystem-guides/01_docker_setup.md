sudo pacman -S --needed docker

alias docker_start="sudo systemctl start docker"
alias docker_status="sudo systemctl status docker"
alias docker_stop="sudo systemctl stop docker"

docker --version

sudo docker run hello-world

_______________________________________________________________________________

sudo groupadd docker

cat /etc/group | grep docker
docker:x:three-digit-number:

_______________________________________________________________________________

Add your username to the group

sudo usermod -aG docker $USER

_______________________________________________________________________________

❯ cat /etc/group | grep docker

docker:x:three-digit-number:your-username

_______________________________________________________________________________

newgrp docker

Now you can just use:

docker run hello-world

_______________________________________________________________________________
