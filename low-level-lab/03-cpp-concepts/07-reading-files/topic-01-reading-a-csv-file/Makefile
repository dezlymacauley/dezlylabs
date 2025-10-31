.SILENT:
.PHONY: run clean

run:
	mkdir -p bin/
	clang++ -std=c++23 -Iinclude src/*.cpp -o bin/main
	./bin/main

clean:
	rm -rf bin/
