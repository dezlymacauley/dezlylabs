
_______________________________________________________________________________
### Blue Nginx

Enter this directory
```sh
cd blue-nginx-container
```

It only has two files:

blue-nginx-container/Dockerfile:
```dockerfile
FROM nginx:1.27.0

RUN apt-get update
RUN apt-get -y install vim

COPY index.html /usr/share/nginx/html/index.html

RUN chown nginx:nginx /usr/share/nginx/html/index.html
```

blue-nginx-container/index.html
```html
<!DOCTYPE html>
<html>
<body>
<h1>Blue Nginx Container</h1>
</body>
</html>
```

Use the Dockerfile in the project to build a Docker Image:
```sh
docker buildx build -t dezlymacauley/web-server-image:blue .
```

Use the Docker Image to create a Docker Container:
```sh
docker run -d -p 127.0.0.1:3000:80 --name blue-one dezlymacauley/web-server-image:blue 
```

Open this in the browser:
```
http://127.0.0.1:3000
```
_______________________________________________________________________________

You can create another container using the same image but on a different port.

Use the Docker Image to create a Docker Container:
```sh
docker run -d -p 127.0.0.1:3500:80 --name blue-two dezlymacauley/web-server-image:blue 
```

Open this in the browser:
```
http://127.0.0.1:3500
```

_______________________________________________________________________________
