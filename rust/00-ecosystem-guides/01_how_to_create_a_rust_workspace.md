
```sh
cargo new name-of-workspace \
--name name-of-workspace \
--vcs none
```

```sh
cd name-of-workspace
```

```sh
touch .gitignore
```

Add this to the `.gitignore` file

.gitignore
```
/target/
```

Create a configuration file for cargo
```sh
mkdir .cargo/
touch .cargo/config.toml
```

Add this to the `.cargo/config.toml` file

.cargo/config.toml
```sh
[alias]
rq = "run --quiet"
```

Create a configuration file for the Rust formatter
```sh
touch rustfmt.toml 
```

Add this to the file:

rustfmt.toml
```sh
max_width = 80
```

Open up the `Cargo.toml` file in the root of the project directory.

Delete the line:
`[dependencies]`


Add this to the end of the file
```sh
[workspace]
resolver = "3"

members = []
[workspace.dependencies]
```
