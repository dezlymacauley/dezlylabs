
```sh
npm init -y
```

Open `package.json`, look for `"type": "commonjs",`
and change it to:
```
"type": "module",
```

Look for `"main": "index.js",`

change it to:
```
"main": "src/index.js",
```

Add this script:
```
"start": "node src/index.js"
```

```sh
npm install express@5.1.0 --save-exact
```

```sh
npm install body-parser@2.2.0 --save-exact
```

To run the project:
```sh
npm run dev 
```

Test with HTTPie:
```sh
http GET http://127.0.0.1:3000
```
_______________________________________________________________________________

To dockerize:

Make sure that you are in this directory where your Dockefile is 
and then run this command:

`--load` Builds the image and loads it into your local Docker engine, 
so you can run it with docker run. For local development and testing,
you almost always want --load.

```sh
docker buildx build --load -t dezlymacauley/expressjs-app:0.0.1 ./
```

To run the docker container:

This will map port 3000 on your local machine, 
with port 3000 on the docker container.

```sh
docker run -d -p 127.0.0.1:3000:3000 \
--name expressjs-app-instance-one dezlymacauley/expressjs-app:0.0.1
```

`--name` is for the container name.

The image is called `express_app`.
_______________________________________________________________________________
