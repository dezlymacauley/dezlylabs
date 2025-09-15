# TypeScript Project Setup
_______________________________________________________________________________

Create the project directory:
```sh
mkdir name-of-project
```
_______________________________________________________________________________

Enter the project directory:
```sh
cd name-of-project
```
_______________________________________________________________________________

Intialize the directory using a minimal bun setup:
```sh
bun init -y -m
```
_______________________________________________________________________________

Create a .gitignore file
```sh
touch .gitignore
```

Run this command to update the file
```sh
cat > .gitignore << 'EOF'
/node_modules/
EOF
```
_______________________________________________________________________________

Create an index.ts file

```sh
touch index.ts
```
_______________________________________________________________________________

To run your code:

```sh
bun index.ts
```
_______________________________________________________________________________
