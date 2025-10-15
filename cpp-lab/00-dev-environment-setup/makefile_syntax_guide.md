# Makefile Syntax Guide
_______________________________________________________________________________
### Add this to the `Makefile`

Makefile
```make
.SILENT:
.PHONY: run clean
 
run:
	mkdir -p bin/
	clang++ src/main.cpp -o bin/main
	./bin/main

clean:
	rm -rf bin/
```
_______________________________________________________________________________
## Understanding the syntax of a Makefile

This is a safegaurd so to ensure that even if there are files in the project
called "run" or "clean", the Makefile will not confuse them for the commands
"run" and "clean" that are in the Makefile.

```
.PHONY: run clean
```

This tells the Makefile not to show the actual commands that it is running,
but instead to only show the output of those commands in the terminal. 
```
.SILENT:
```

This command `make run` will do create a directory called `bin/` if it does
not exist.
```
run:
    mkdir -p bin/
	clang++ src/main.cpp -o bin/main
    ./bin/main
```

`clang++ src/main.cpp -o bin/main` means that clang++ will be used 
to compile `src/main.cpp` into and executable binary that will be saved in
`bin/main`

_______________________________________________________________________________
