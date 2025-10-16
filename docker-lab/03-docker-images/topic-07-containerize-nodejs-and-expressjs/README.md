
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
