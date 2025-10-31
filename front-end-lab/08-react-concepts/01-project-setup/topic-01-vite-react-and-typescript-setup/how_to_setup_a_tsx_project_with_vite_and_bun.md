# How to setup a TSX (TypeScript React) project with Bun and Vite
_______________________________________________________________________________
### Step 1 - Create the project directory

Navigate to the location where you want your project directory to be created,
then run this command:
```sh
mkdir name-of-project
```

Enter the project directory
```sh
cd name-of-project
```
_______________________________________________________________________________

Run this command to create the project structure in the current directory
```sh
bun create vite ./ \
--template react-ts
```
_______________________________________________________________________________

Choose yes here as this will soon be the default.

```
◆  Use rolldown-vite (Experimental)?:
│  ● Yes (The future default Vite, which is powered by Rolldown)
│  ○ No
└
```
_______________________________________________________________________________

Choose no. This is because I want to use version fox to specify the version
of bun that I want my project to use, and also because I want to pass
the `--exact` flag to the `bun install` command.

```
◆  Install with bun and start now?
│  ○ Yes / ● No

```

_______________________________________________________________________________

### Project Customization
_______________________________________________________________________________

#### `package.json`

Open the `package.json` file and look for this line:
```
"dev": "vite",
```
_______________________________________________________________________________

Change it to:
```json
"dev": "vite --open --host 127.0.0.1 --port 6969",
```

Now when you run the command `bun run dev`, 
it automatically open up your default web browser and start the development 
server on: `http://127.0.0.1:6969`

_______________________________________________________________________________

You can also change this:
```json
"preview": "vite preview"
```

to this:
```json
"preview": "vite preview --open --host 127.0.0.1 --port 7070"
```
_______________________________________________________________________________

Finally, go through the `package.json` and remove all of the symbols `^`,
and `~`

E.g. Change this:
```
"react": "^19.1.1",
```

To this:
```
"react": "19.1.1",
```

Tip: You can use this substitution macro in Neovim
```
:%s/\^//gc
```

```
:%s/\~//gc
```

This will ensure that that the exact versions of dependencies are used
when I run `bun install --exact`
_______________________________________________________________________________

## TODO: Add a guide for Version Fox

Run this command
```sh
vfox use -p bun@1.2.2
```

This will create a `.tool-versions` file that has the version of Bun 
that you are using.

_______________________________________________________________________________

Use this is install the exact version of dependencies using Bun
```sh
bun install --exact
```
_______________________________________________________________________________


Press `ctrl + c` to close the development server.
