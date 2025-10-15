# Python Project With Virtual Environment 
_______________________________________________________________________________
## Step 1 - Install a `uv managed` Python version for your project 

You can check out which one to download from here:
```
https://www.python.org/downloads/
```

For this guide I want to install Python version `3.14.0`
```sh
cd $HOME
uv python install 3.14.0
```

The format is `major version`, `minor version`, `patch version`

E.g. `3.14.0` major version is 3, minor version, is 14, and patch version is 0

To view the list of installed Python versions that are managed by uv:
run this command:
```sh
uv python list --only-installed --managed-python
```
uv allows you to install multiple versions of Python.

If you ever want to remove a version of Python, then do this:
```sh
uv python uninstall 3.14.0
```
_______________________________________________________________________________
## Step 2 - Create a new project directory and enter it

Replace `name-of-project` with your actual project name

```sh
mkdir name-of-project
cd name-of-project
```
_______________________________________________________________________________
## Step 3 - Setup the project directory

For the rest of this guide, all commands must be run 
from the project directory.

_______________________________________________________________________________
### Create a `pyproject.toml` file

```sh
uv init --bare --python 3.14.0
```
_______________________________________________________________________________
### Create a `.python-version`

Create a `.python-version` file
```sh
touch .python-version
```

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

Add this to the file
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

# Temporary backup copies of notebooks
.ipynb_checkpoints/
EOF
```
_______________________________________________________________________________
### Create a Python virtual environment

Create a Python virtual environment that uses the uv-managed version 
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
direnv: error /home/dezlymacauley/dezly-macauley/computer-science/
python/accumulated-skills/lesson-01-classes-and-objects/.envrc 
is blocked. Run `direnv allow` to approve its content
```

Run this command to tell direnv to automatically activate 
the virtual environment when you enter this directory,
automatically deactivate it when you are not using this directory.
```sh
cat > .envrc << 'EOF'
source .venv/bin/activate
EOF
```

Finally give direnv permission to load the contents of the `.envrc` file.

Run this command
```sh
direnv allow
```
_______________________________________________________________________________
Confirm that your project is using the Python version from 
the virtual environment.

```sh
which python
```

You should see something like this
```
/home/dezlymacauley/dezly-macauley/computer-science/python/
accumulated-skills/lesson-01-classes-and-objects/.venv/bin/python
```
_______________________________________________________________________________
## Step 4 - Install the AI / ML Python packages

Install the project dependencies

```sh
uv add pandas
```

```sh
uv add numpy
```

```sh
uv add scikit-learn 
```

_______________________________________________________________________________

Install the development dependencies

```sh
uv add --dev jupyterlab-vim
```

```sh
uv add --dev matplotlib
```

```sh
uv add --dev httpie==3.2.4
```
_______________________________________________________________________________
## Step 4 - Add ruff formatting settings to your `pyproject.toml` 

Add the `[tool.ruff]` and `[tool.ruff.format]` 
to configure the `ruff formatter`.

The list of settings can be found here:

```
https://docs.astral.sh/ruff/configuration/
```

`pyproject.toml`
```toml
[project]
name = "sample-project"
version = "0.1.0"
requires-python = ">=3.13.7"
dependencies = [
    "numpy>=2.3.2",
    "pandas>=2.3.1",
    "scikit-learn>=1.7.1",
]

[dependency-groups]
dev = [
    "jupyter>=1.1.1",
    "matplotlib>=3.10.5",
]

[tool.ruff]
line-length = 80
indent-width = 4

[tool.ruff.format]
quote-style = "double"
indent-style = "space"
```
_______________________________________________________________________________
### Create a directory for csv data

```sh
mkdir csv-data
```
_______________________________________________________________________________
### Using jupyter notebooks

```sh
mkdir jupyterlab-notebooks
```

To use:

Open a separate terminal and launch a Jupyter Notebook 
in your default browser.

```sh
cd jupyterlab-notebooks
jupyter lab
```
_______________________________________________________________________________
## How to Rebuild this project after cloning

```sh
uv sync
```

```sh
direnv allow
```
_______________________________________________________________________________
