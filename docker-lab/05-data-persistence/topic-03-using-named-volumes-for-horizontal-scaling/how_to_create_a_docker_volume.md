
```sh
docker volume create name-of-volume
```

To view a list of docker volumes:
```sh
docker_volumes
```

This volume will be created in:
```sh
/var/lib/docker/volumes/
```

How to check if a container is using a volume before you delete that volume:
```sh
docker ps -a --filter volume=name-of-volume
```

To remove a volume:
```sh
docker volume rm name-of-volume
```
_______________________________________________________________________________

How to Add a volume to a container

```sh
docker run -d -p 127.0.0.1:3000:80 --name website-main -v website-data:/usr/share/nginx/html nginx:1.27.0
docker run -d -p 127.0.0.1:3001:80 --name website-readonly1 -v website-data:/usr/share/nginx/html nginx:1.27.0
docker run -d -p 127.0.0.1:3002:80 --name website-readonly2 -v website-data:/usr/share/nginx/html nginx:1.27.0
docker run -d -p 127.0.0.1:3003:80 --name website-readonly3 -v website-data:/usr/share/nginx/html nginx:1.27.0
```

The names are actually misleading because changing the data in any 
of these containers will affect the others.

_______________________________________________________________________________

```sh
docker exec -it website-main sh
```

```sh
echo "Dezly Is Awesome" > /usr/share/nginx/html/index.html
```

_______________________________________________________________________________

docker exec -it website-readonly1 sh
_______________________________________________________________________________

If you change the `index.html` in one container, 
all of the other containers will be updated because they 
are sharing the same volumes.
_______________________________________________________________________________

This is called horizontal scalling
_______________________________________________________________________________
