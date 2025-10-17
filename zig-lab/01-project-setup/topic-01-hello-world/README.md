```sh
mkdir name-of-project
```

```sh
cd name-of-project
```

```sh
zig init
```

```sh
touch .gitignore
```

Add this to the .gitignore file
```
# Zig build cache
.zig-cache/

# Executable binary
zig-out/
```
_______________________________________________________________________________

```
.
├── build.zig
├── build.zig.zon
├── README.md
└── src
    ├── main.zig
    └── root.zig
```

The `src` directory contains two modules: `main.zig` and `root.zig`

`main.zig` is the main module of an executable Zig program.

`root.zig` tells Zig where to look for any custom internal modules that
may be used in `main.zig`

_______________________________________________________________________________

`build.zig` This is a build script in Zig that is used to build the entire
project.

`build.zig.zon` This is where external dependencies are listed. 

_______________________________________________________________________________
### To build your project 

```sh
zig build
```

_______________________________________________________________________________
