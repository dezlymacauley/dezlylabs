```sh
mkdir name-of-project
cd name-of-project
```

```sh
bun init --minimal --yes
```

This will create the following:

node_modules/
bun.lock
package.json
tsconfig.json


```sh
mkdir -p src
touch src/main.ts
touch README.md
touch .gitignore
touch .dockerignore
```

```sh
bun add express@5.1.0 --exact
bun add @types/express@5.0.3 --dev --exact
bun add typescript@5.9.3 --peer --exact
```

package.json
```json
{
  "devDependencies": {
    "@types/bun": "1.2.2",
    "@types/express": "5.0.3"
  },
  "dependencies": {
    "express": "5.1.0"
  },
  "peerDependencies": {
    "typescript": "5.9.3"
  }
}
```

I changed `"@types/bin": "latest"` to `"@types/bun": "1.2.2"`
because that is the version of Bun I specified in the `Dockerfile`
