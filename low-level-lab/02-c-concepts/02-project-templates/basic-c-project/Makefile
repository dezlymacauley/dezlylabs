.SILENT:
.PHONY: run clean

run:
	mkdir -p bin/
	gcc -std=c23 -Iinclude src/*.c -o bin/main
	./bin/main

clean:
	rm -rf bin/
