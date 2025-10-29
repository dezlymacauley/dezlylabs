# How to create a Basic Tailwind CSS project using Bun and Vite
_______________________________________________________________________________

Create the project directory
```sh
mkdir bun-sveltekit-ts-tw-project
```
_______________________________________________________________________________

Enter the project directory
```sh
cd bun-sveltekit-ts-tw-project
```
_______________________________________________________________________________

Create a new project using `Bun` and `SvelteKit`

```sh
bunx sv create ./
```
_______________________________________________________________________________

Select this option
```
◆  Which template would you like?
│  ● SvelteKit minimal (barebones scaffolding for your new app)
│  ○ SvelteKit demo
│  ○ Svelte library
└
```

Select this option
```
◆  Add type checking with TypeScript?
│  ● Yes, using TypeScript syntax
│  ○ Yes, using JavaScript with JSDoc comments
│  ○ No
└
```

Select `tailwindcss`
```
◆  What would you like to add to your project?
(use arrow keys / space bar)
│  ◻ prettier
│  ◻ eslint
│  ◻ vitest
│  ◻ playwright
│  ◼ tailwindcss (css framework -
https://tailwindcss.com)
│  ◻ sveltekit-adapter
│  ◻ devtools-json
│  ◻ drizzle
│  ◻ lucia
│  ◻ mdsvex
│  ◻ paraglide
│  ◻ storybook
│  ◻ mcp
└
```

Select all the Tailwind CSS plugins
```
◆  Which plugins would you like to add?
│  ◼ typography (@tailwindcss/typography)
│  ◼ forms (@tailwindcss/forms)
└
```

Select `None`
```
◆  Which package manager do you want to install
dependencies with?
│  ● None
│  ○ npm
│  ○ yarn
│  ○ pnpm
│  ○ bun
│  ○ deno
└
```
_______________________________________________________________________________

This will create the following:

```
src/
static/
.gitignore
.npmrc
README.md
package.json
svelte.config.js
tsconfig.json
vite.config.ts
```
_______________________________________________________________________________
#### `.gitignore`

Replace the contents of the file with this:
```gitignore
# Project dependencies
/node_modules/

# Everything SvelteKit needs to run your application during development
/.svelte-kit/
```
_______________________________________________________________________________
#### `README.md`

Clear out the contents

_______________________________________________________________________________

#### `package.json`

Add this to you `package.json`, right after the line `"type": "module"`
```json
"type": "module",
"engines": {
 "bun": "1.3.1"
},
```
_______________________________________________________________________________

Add a `clean` command to your `package.json`, in the scripts section
```sh
"dev": "vite --open --host 127.0.0.1 --port 6969",
"clean": "rm -rf node_modules .svelte-kit",
```
_______________________________________________________________________________

Add a `setup` command to your `package.json`

```sh
"clean": "rm -rf node_modules .svelte-kit",
"setup": "bun install --exact && bun pm trust @tailwindcss/oxide || true",
```

The `&&` is not a typo. This is a bash command that means 
the second should only run if the first command succeeded.
_______________________________________________________________________________

Look for the line 

```json
"dev": "vite",
```

And change it to:
```json
"dev": "vite --open --host 127.0.0.1 --port 6969",
```
_______________________________________________________________________________

Remove all of the `~` and `^` symbols that appear before 
any version numbers in the file.


E.g. `"typescript": "^5.39.5"` becomes

```
"typescript": "5.9.3"
```

I like my package.json to contain exact versions of tools,
for maximum reproducibility of the project.

_______________________________________________________________________________

Add `@types/bun` to the `devDependencies` section:

```
"@tailwindcss/vite": "4.1.13",
"@types/bun": "1.3.1",
```

This is to help TypeScript understand how to work with any bun specific 
variable types you may use in your project.
_______________________________________________________________________________
### Install the project dependencies

```sh
bun install --exact --trust
```
_______________________________________________________________________________
### Blocked postinstall scripts

If you see a message like this:

```sh
Blocked 1 postinstall. Run `bun pm untrusted` for details.
```

Run this command:
```sh
bun pm untrusted
```
_______________________________________________________________________________

This will show you the exact dependency that bun has blocked.
```
./node_modules/@tailwindcss/oxide @4.1.13
 » [postinstall]: node ./scripts/install.js
```

This is safe to use as it's from the Tailwind CSS team.

Run this command
```sh
bun pm trust @tailwindcss/oxide
```

Note: Dont add the version number or it will fail.

You should see an output like this:
```
1 script ran across 1 package
```
_______________________________________________________________________________
#### `vite.config.ts`

Replace the contents of the file with this:
```ts
import tailwindcss from "@tailwindcss/vite";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()]
});
```
_______________________________________________________________________________
#### `src/app.css`

Replace the contents of the file with this:
```css
@import "tailwindcss";
@plugin "@tailwindcss/forms";
@plugin "@tailwindcss/typography";
```
_______________________________________________________________________________
#### `src/routes/+page.svelte`

Clear out the contents of the page
_______________________________________________________________________________
### To run the project

Open up a separate terminal and run 

```sh
bun run dev
```

_______________________________________________________________________________

### To save disk space when you are not working on the project

```sh
bun run clean
```
_______________________________________________________________________________

### To setup the project if you have just cloned this repo or cleaned it

```sh
bun run setup
```

If you see this message it is because the postinstall script
for `@tailwindcss/oxide` has already been trusted before
```
error: 0 scripts ran. The following packages are already trusted, 
don't have scripts to run, or don't exist:

 - @tailwindcss/oxide
```

You can search for the `@tailwindcss/oxide` package in `bun.lock` to confirm
that it has been installed.

_______________________________________________________________________________
