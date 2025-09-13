# TypeScript Lab Setup
_______________________________________________________________________________

```sh
mkdir name-of-project
cd name-of-project
```
_______________________________________________________________________________

```sh
bun init -y -m
```
_______________________________________________________________________________

```sh
touch .gitignore
```

```sh
cat > .gitignore << 'EOF'
/node_modules/
EOF
```
_______________________________________________________________________________

```sh
mkdir -p src/
touch src/index.ts
```

```sh
cat > src/index.ts << 'EOF'
console.log("Hello via Bun!");
EOF
```
_______________________________________________________________________________

```sh
cat > package.json << 'EOF'
{
  "name": "name-of-project",
  "version": "1.0.0",
  "devDependencies": {
    "@types/bun": "latest"
  },
  "scripts": {
    "r": "bun --bun run src/index.ts",
    "w": "bun --bun --watch run src/index.ts",
    "c": "rm -rf node_modules/"
  }
}
EOF
```
_______________________________________________________________________________
### Commands

`bun run r` This will `run` the file `src/index.ts` once.
`bun run w` This will `watch` the file `src/index.ts` for changes.
_______________________________________________________________________________

`bun run c` This will `clean` the `node_modules/` directory.

To re-install the dependencies use `bun install`

_______________________________________________________________________________
