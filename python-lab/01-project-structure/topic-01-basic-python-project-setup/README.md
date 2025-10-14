# Basic Python Project Setup
_______________________________________________________________________________
### Decide what version of Python you want your project to use

You can view the version number of the latest release of Python here,
or scroll down to see what other versions are available:
[Latest Python version](https://www.python.org/downloads/)

I want to use Python **3.14.0** for this guide.

_______________________________________________________________________________
### Use uv to install the latest version of Python

You want to run this command in your home directory 
and not your project directory as this is a global install.
```sh
cd $HOME
uv python install 3.14.0
```
The format is `major version`, `minor version`, `patch version`

E.g. `3.14.0` major version is 3, minor version, is 14, and patch version is 0

_______________________________________________________________________________
To view the list of installed Python versions that are managed by uv:
run this command:

```sh
uv python list --only-installed --managed-python
```

If you ever want to remove a version of Python that you no longer need, 
then run this command:
```sh
uv python uninstall 3.14.0
```
_______________________________________________________________________________
### Create a new project directory and enter it

```sh
mkdir name-of-project
cd name-of-project
```

For the rest of the guide, run these commands from inside this directory.
_______________________________________________________________________________
### Intialize the project by creating a `pyproject.toml` file

```sh
uv init --bare --python 3.14.0
```

Add this to the end of your `pyproject.toml`

```toml
[tool.ruff]
line-length = 80
indent-width = 4

[tool.ruff.format]
quote-style = "double"
indent-style = "space"
```
_______________________________________________________________________________
### Create a `.python-version` file

This file is used alongside `pyproject.toml` to make your Python projects 
reproducible when you need to rebuild them.

Run this command to edit the file
```sh
cat > .python-version << 'EOF'
3.14.0
EOF
```
_______________________________________________________________________________
### Create a `main.py` file

```sh
touch main.py
```

Edit the file
```sh
cat > main.py << 'EOF'
def main():
    pass


if __name__ == "__main__":
    main()
EOF
```
_______________________________________________________________________________
### Create a `.gitignore` file

```sh
touch .gitignore
```

Edit the file
```sh
cat > .gitignore << 'EOF'
# The Python virtual environment
venv/

# Used by ruff to speed up code formatting
.ruff_cache/ 
EOF
```
_______________________________________________________________________________
### Create a Python virtual environment

Create a Python virtual environment that uses the `uv-managed` version 
of Python that you installed.

```sh
uv venv --python 3.14.0
```
_______________________________________________________________________________
### Create a `.envrc` file to automatically load the virtual environment

```sh
touch .envrc
```

You'll see a message like this. Ignore it for now
```
direnv: error /home/..../.envrc is blocked. 
Run `direnv allow` to approve its content
```

Edit the file
```sh
cat > .envrc << 'EOF'
source .venv/bin/activate
EOF
```

You'll see this message again.
```
direnv: error /home/..../.envrc is blocked. 
Run `direnv allow` to approve its content
```

Run this command:
```sh
direnv allow
```
_______________________________________________________________________________
### Create a `Makefile`

```sh
touch Makefile
```

Edit the file
```sh
cat > .Makefile << 'EOF'
.SILENT:
.PHONY: run clean setup

run:
	python main.py

clean:
	rm -rf .venv/

setup: clean
	uv sync && direnv allow
EOF
```
_______________________________________________________________________________
### If you ever need to rebuild your project just run this command

```sh
make clean
```
_______________________________________________________________________________
