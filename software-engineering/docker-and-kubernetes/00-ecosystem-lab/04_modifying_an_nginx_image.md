
_______________________________________________________________________________

```sh
sudo systemctl start docker
```
_______________________________________________________________________________

```sh
docker pull nginx:1.27.0
```

_______________________________________________________________________________

```sh
docker create \
-p 127.0.0.1:8080:80/tcp \
--name web_server \
nginx:1.27.0
```
_______________________________________________________________________________

```sh
docker start web_server
```
_______________________________________________________________________________

Use the program `httpie` to send a GET request.

```sh
http GET http://127.0.0.1:8080
```

You should get an HTTP response back from the Nginx server.
```
HTTP/1.1 200 OK
Accept-Ranges: bytes
Connection: keep-alive
Content-Length: 615
Content-Type: text/html
Date: Sun, 14 Sep 2025 06:55:29 GMT
ETag: "6655da96-267"
Last-Modified: Tue, 28 May 2024 13:22:30 GMT
Server: nginx/1.27.0

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

_______________________________________________________________________________

Nginx is listening on port 80 inside the container.

Docker mapped it to 127.0.0.1:8080 on your host.

### The headers (everything before the HTML)

These are HTTP response headers sent by Nginx:

HTTP/1.1 200 OK → The status line.

200 OK means the request was successful.

Server: nginx/1.27.0 → The server that generated the response (Nginx, version 1.27.0).

Date: Sun, 14 Sep 2025 06:55:29 GMT → When the server created this response.

Content-Type: text/html → The type of data being sent (HTML in this case).

Content-Length: 615 → The size of the body (in bytes).

Last-Modified & ETag → Used for caching (so clients don’t download unchanged files again).

Connection: keep-alive → Tells the client to keep the TCP connection open for future requests.

_______________________________________________________________________________

```
🔹 The body (everything after the headers)

That’s just a simple HTML file — the default Nginx welcome page that ships with the image.

<!DOCTYPE html> → Says the document is HTML5.

<title>Welcome to nginx!</title> → Title shown in the browser tab.

<h1>Welcome to nginx!</h1> → Big header text in the page.

<p>…</p> → Some explanatory text, links to Nginx docs, and a thank-you note.

👉 If you opened http://127.0.0.1:8080 in a browser, you’d see the famous Nginx welcome screen.🔹 The body (everything after the headers)

That’s just a simple HTML file — the default Nginx welcome page that ships with the image.

<!DOCTYPE html> → Says the document is HTML5.

<title>Welcome to nginx!</title> → Title shown in the browser tab.

<h1>Welcome to nginx!</h1> → Big header text in the page.

<p>…</p> → Some explanatory text, links to Nginx docs, and a thank-you note.

👉 If you opened http://127.0.0.1:8080 in a browser, you’d see the famous Nginx welcome screen.🔹 The body (everything after the headers)

That’s just a simple HTML file — the default Nginx welcome page that ships with the image.

<!DOCTYPE html> → Says the document is HTML5.

<title>Welcome to nginx!</title> → Title shown in the browser tab.

<h1>Welcome to nginx!</h1> → Big header text in the page.

<p>…</p> → Some explanatory text, links to Nginx docs, and a thank-you note.
```

You can also see this by opening http://127.0.0.1:8080 in a browser

_______________________________________________________________________________

Run the `bash` program from an interactive shell `-it` inside the
nginx container called `web_server`
```sh
docker exec -it web_server bash
```

Your prompt will now look like this. That number is the container id
```
root@47a2ed9530e5:/#
```

_______________________________________________________________________________

You can run `ls` to list the contents of the root directory of the 
docker container.

If you run `pwd` it should return `/` which means root directory.

You are also in the bash shell as a root user so you don't need to use `sudo`,
because you have full access to the container as root.

The reason why `bash` and `ls` work is because the `nginx` image is 
actually built ontop of a docker image.


You can confirm this with this command:
```sh
cat /etc/os-release
```

_______________________________________________________________________________

Update the Debian package manager in the container
```sh
apt-get update
```

To search for a package the `apt-cache search ^name-of-package$`

The `^` at the start and `$` at the end mean exact match
```sh
apt-cache search ^bat$
```

```
apt-cache search ^neovim$
```

I can install neovim by running this command:
```sh
apt-get install neovim bat
```
_______________________________________________________________________________

View the default `index.html` file of Nginx using `bat`

On Debian the package is `bat` however you binary executable is `batcat`
```sh
batcat /usr/share/nginx/html/index.html
```
_______________________________________________________________________________

To edit this file:

```sh
nvim /usr/share/nginx/html/index.html
```

```html
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

_______________________________________________________________________________
