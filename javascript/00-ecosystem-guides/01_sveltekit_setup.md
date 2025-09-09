# SvelteKit Setup
_______________________________________________________________________________

```sh
mkdir name-of-project
```

```sh
cd name-of-project
```
_______________________________________________________________________________

```sh
npx sv create
```

Sometimes you'll see this
```
Need to install the following packages:
sv@0.9.4
Ok to proceed? (y)
```

Type `y` and press `Enter`

_______________________________________________________________________________

You'll see this
```
┌  Welcome to the Svelte CLI! (v0.9.4)
│
◆  Where would you like your project to be created?
│    (hit Enter to use './')
└
```

Press Enter to create the project in the current directory.

_______________________________________________________________________________

```
◆  Which template would you like?
│  ● SvelteKit minimal (barebones scaffolding for your new
app)
│  ○ SvelteKit demo
│  ○ Svelte library
└
```

Select `SvelteKit minimal` and press `Enter`
_______________________________________________________________________________

Select TypeScript and press Enter 
```
◆  Add type checking with TypeScript?
│  ● Yes, using TypeScript syntax
│  ○ Yes, using JavaScript with JSDoc comments
│  ○ No
└
```
_______________________________________________________________________________

Here are the available options. 

Use the arrow keys to move arround then use the space bar to select `prettier` 
and `eslint`, then press `Enter` to continue.

```
◆  What would you like to add to your project? (use arrow
keys / space bar)
│  ◻ prettier (formatter - https://prettier.io)
│  ◻ eslint
│  ◻ vitest
│  ◻ playwright
│  ◻ tailwindcss
│  ◻ sveltekit-adapter
│  ◻ devtools-json
│  ◻ drizzle
│  ◻ lucia
│  ◻ mdsvex
│  ◻ paraglide
│  ◻ storybook
└
```
_______________________________________________________________________________

Select npm, and press Enter. I've tried other package managers and
they will all be faster as installing things than npm, 
but most tools are tested for npm first, so once it is done I now that I won't
have any issues for the rest of my project setup.

```
◆  Which package manager do you want to install dependencies
with?
│  ○ None
│  ● npm
│  ○ yarn
│  ○ pnpm
│  ○ bun
│  ○ deno
└
```

_______________________________________________________________________________

Open up `package.json` and look for the `"scripts" section.

You should see a command called "dev"

Replace this line: `"dev": "vite dev",
 `

With this: `"dev": "vite dev --open --port 7399`

_______________________________________________________________________________
### .prettierrc

Change these three lines:
```
"singleQuote": true,
"trailingComma": "none",
"printWidth": 100,
```

To this:
```
"singleQuote": false,
"trailingComma": "none",
"printWidth": 80,
```

_______________________________________________________________________________

Open a separate terminal and run the command:

```sh
npm run dev
```

_______________________________________________________________________________
