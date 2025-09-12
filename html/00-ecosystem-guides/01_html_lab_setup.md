# HTML Lab Setup
_______________________________________________________________________________
## Intro: Why I chose `bun` and `SvelteKit` to create my HTML Lab

1. Bun is a faster alternative to Node.js when it comes to installing 
packages like Svelte and SvelteKit.

2. Svelte is a front-end JavaScript framework that takes 
a minimalist approach to web development by building ontop of what makes HTML
great, rather than trying to squeeze HTML in JavaScript like React does with
JSX and TSX.

Everything that is inside the `<body></body>` tag of a regular HTML file,
is valid inside a Svelte file. 

So focusing on this part alone, 
that makes Svelte essentially a superset of HTML.

3. SvelteKit is a JavaScript meta-framework.
A JavaScript meta-framework is framework that adds functionality 
to an existing JavaScript framework.

SvelteKit makes it more convinient to use Svelte. 

Here are some of the features from Svelte that I will be using in my HTML Lab.

- File-based routing 
This makes it really easy to create nested routes for HTML pages, 
so this will make it easier to group things into sections and topics.

- Custom Import Alias
This makes it really easy to create a aliases that make it easy to import
files from the project structure. 

I will be using thes feature so that multiple HTML files 
can share the same resources.

_______________________________________________________________________________
## Step 1: Create a new SvelteKit project

Create a directory for the project.
```sh
mkdir name-of-project
```
_______________________________________________________________________________

Enter the project directory:
```sh
cd name-of-project
```

For the rest of this guide, 
all commands must be run from the root directory of the project.

_______________________________________________________________________________

Run this command to create the basic structure of a SvelteKit project.
```sh
bunx sv create . \
--template minimal \
--no-types \
--install bun
```

_______________________________________________________________________________

Use the `arrow keys` and the `space bar` to select `prettier`, 
then press `Enter`.

```sh
◆  What would you like to add to your project? (use arrow keys / space bar)
│  ◼ prettier (formatter - https://prettier.io)
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

Confirm that the project can be built successfully.
```sh
bun run build
```
_______________________________________________________________________________

This is what the root of your project should look like:

```
.svelte-kit/
node_modules/
src/
static/
.gitignore
.npmrc
.prettierignore
.prettierrc
README.md
bun.lock
jsconfig.json
package.json
svelte.config.js
vite.config.js
```

This is the initial project structure.

```
.
├── bun.lock
├── .gitignore
├── jsconfig.json
├── .npmrc
├── package.json
├── .prettierignore
├── .prettierrc
├── README.md
├── src
│   ├── app.html
│   ├── lib
│   │   ├── assets
│   │   │   └── favicon.svg
│   │   └── index.js
│   └── routes
│       ├── +layout.svelte
│       └── +page.svelte
├── static
│   └── robots.txt
├── svelte.config.js
└── vite.config.js
```
_______________________________________________________________________________
## Step 2: Customizing the project

Run all of these commands from the root directory of the project.

_______________________________________________________________________________
### `src/routes/+page.svelte`

Clear the contents of the home page
```sh
truncate -s 0 src/routes/+page.svelte
```
_______________________________________________________________________________
### Create a global css stylesheet

```sh
touch src/app.css
```

Run this command 
```
cat > src/app.css << 'EOF'
:root {
  --dezly-lab-black: #0a0a0c;
  --dezly-lab-purple: #cd40f8;
}

body {
  background-color: var(--dezly-lab-black);
  color: white;
}

a {
  color: var(--dezly-lab-purple); 
  font-weight: bold;
}
EOF
```
_______________________________________________________________________________
### `src/routes/+layout.svelte`

Clear the contents of the file page
```sh
truncate -s 0 src/routes/+layout.svelte
```

Run this command
```sh
cat > src/routes/+layout.svelte << 'EOF'
<script>
	import favicon from "$lib/assets/favicon.svg";
	import "../app.css";

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

{@render children?.()}
EOF
```
_______________________________________________________________________________

### `README.md`

Clear the contents of the file:
```sh
truncate -s 0 README.md
```
_______________________________________________________________________________
### `package.json`

         Change this:
		"dev": "vite dev",
		"preview": "vite preview",

         To this:
		"dev": "vite dev --open",
		"preview": "vite preview --open",

_______________________________________________________________________________
### `.prettierrc`


         Change this:
	"singleQuote": false,
	"printWidth": 80,


         To this:
	"singleQuote": false,
	"printWidth": 80,

_______________________________________________________________________________
## Step 3: Run the development server

Run this command to open the development server on port 6969.
The page should automatically open up in the browser.

```sh
bun run dev
```
_______________________________________________________________________________
