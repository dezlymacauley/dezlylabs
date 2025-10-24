# How to create a basic Rust project
_______________________________________________________________________________

Create the project directory
```sh
mkdir name-of-project
```

Enter the project directory
```sh
cd name-of-project
```
_______________________________________________________________________________

Intialize the project
```sh
cargo init
```
_______________________________________________________________________________
### `.cargo/config.toml`

Create the `.cargo` directory
```sh
mkdir .cargo  
```

Create a `config.toml` file. This will allow you 
to create aliases for cargo commands.

touch
```sh
touch .cargo/config.toml
```

Add this to the file
```toml
[alias]
rq = "run --quiet"
```
_______________________________________________________________________________
### `.gitignore`

Create a `.gitignore` file if it does not exist in the project
```sh
touch .gitignore 
```

Add this to the file:
```gitignore
/target/
```

This will ensure that the target directory is not pushed up to GitHub.

The `target` directory is where your compiled binaries will be placed.
_______________________________________________________________________________
### `rustfmt.toml`

```sh
touch rustfmt.toml
```
_______________________________________________________________________________

To run the program:
```sh
cargo rq
```
_______________________________________________________________________________

To save space on your local machine:

```sh
cargo clean
```

This will delete the `target` directory.

_______________________________________________________________________________
