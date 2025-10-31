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
```

package.json
```json
{
  "devDependencies": {
    "@types/bun": "1.3.0",
    "@types/express": "5.0.3"
  },
  "dependencies": {
    "express": "5.1.0"
  }
}
```

I changed `"@types/bin": "latest"` to `"@types/bun": "1.3.0"`
because that is the version of Bun I specified in the `Dockerfile`
