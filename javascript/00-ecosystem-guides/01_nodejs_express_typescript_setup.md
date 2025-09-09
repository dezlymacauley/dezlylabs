# Node.js, Express, and TypeScript Setup
_______________________________________________________________________________

Create the project directory and enter it
```sh
mkdir name-of-project
cd name-of-project
```
_______________________________________________________________________________

Create a TypeScript file called `main.ts`

```sh
touch main.ts
```
_______________________________________________________________________________

Create a `.gitignore` file
```sh
touch .gitignore
```

Open the `.gitignore` file and add this to the file

```
/node_modules/
```

_______________________________________________________________________________

Create a `package.json` file
```sh
npm init -y
```

Open the file.

Change `"main": "index.js",` to `"main": "src/index.ts"`

Change `"type": "commonjs",` to `"type": "module"`

_______________________________________________________________________________

Run this command to create a `node_modules` directory 
and a `package-lock.json` file.  

```sh
npm install
```
_______________________________________________________________________________

Install the project dependencies

```sh
npm install --save-dev typescript
npm install --save-dev @types/express
```

_______________________________________________________________________________
