# How to setup a TSX (TypeScript React) project with Bun and Vite
_______________________________________________________________________________
### Step 1 - Create the project directory

Navigate to the location where you want your project directory to be created,
then run this command:

_______________________________________________________________________________

```sh
bun create vite name-of-project \
--template react-ts
```

bun create vite bun-vite-react-typescript \
--template react-ts
_______________________________________________________________________________

Choose yes here as this will soon be the default.

```
◆  Use rolldown-vite (Experimental)?:
│  ● Yes (The future default Vite, which is powered by Rolldown)
│  ○ No
└
```
_______________________________________________________________________________

Choose yes to install the project dependencies 
and start the development server.

```
◆  Install with bun and start now?
│  ● Yes / ○ No
└
```

After it is done open a web browser and paste this in to view your website.

```
http://localhost:5173/
```

Press `ctrl + c` to close the development server.

_______________________________________________________________________________

To start the server again, open a separate terminal and run this command:
```sh
bun dev
```

Press `ctrl + c` to close the development server.

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
```
"dev": "vite --open --host 127.0.0.1 --port 6969",
```

Now when you run the command `bun dev`, 
it automatically open up your default web browser and start the development 
server on: `http://127.0.0.1:6969`

_______________________________________________________________________________
