
```sh
docker buildx build -t dezlymacauley/nginx-and-vim:1.2.7 .
```

To run the Docker container using this image

```sh
docker run -d -p 127.0.0.1:8080:80 dezlymacauley/nginx-and-vim:1.2.7
```

`-d` means detached. The container will run in the background

```sh
docker_containers_status
```

The container is up
```sh
CONTAINER ID   NAMES               STATUS
99de35735758   nostalgic_hypatia   Up 17 seconds
```

To enter the container via an interactive bash shell:
```sh
docker exec -it nostalgic_hypatia bash
```

Open this in the browser to see the page.
```sh
http://127.0.0.1:8080/
```
