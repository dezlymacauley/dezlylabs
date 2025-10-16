
```sh
bun create vite name-of-project \
--template react-ts
```

Go to `package.json`

and look for the line:
```
"dev": "vite",
```

Replace it with:
```
"dev": "vite --open --host 127.0.0.1 --port 6969",
```

Add this to the end of your `.gitignore` file
```sh
# Environment Variables
.env
```

To start the project, open a separate terminal an run:
```sh
bun run dev
```
