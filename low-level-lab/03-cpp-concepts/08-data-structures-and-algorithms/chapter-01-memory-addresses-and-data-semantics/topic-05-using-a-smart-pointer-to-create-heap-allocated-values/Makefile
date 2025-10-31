# .SILENT:
# .PHONY: run clean
#
# run:
# 	mkdir -p bin/
# 	clang++ -std=c++23 -Iinclude src/*.cpp -o bin/main
# 	./bin/main
#
# clean:
# 	rm -rf bin/
#
#______________________________________________________________________________

# .PHONY: run clean
#
# CXX = clang++
# CXXFLAGS = -std=c++23 -Iinclude
# SRC = $(wildcard src/*.cpp)
# OBJ = $(SRC:src/%.cpp=bin/%.o)
# TARGET = bin/main
#
# $(TARGET): $(OBJ)
# 	$(CXX) $(CXXFLAGS) $^ -o $@
#
# bin/%.o: src/%.cpp
# 	mkdir -p bin
# 	$(CXX) $(CXXFLAGS) -c $< -o $@
#
# run: $(TARGET)
# 	./$(TARGET)
#
# clean:
# 	rm -rf bin
#
#

.PHONY: run clean default
default: run

CXX = clang++
CXXFLAGS = -std=c++23 -Iinclude
SRC = $(wildcard src/*.cpp)
OBJ = $(SRC:src/%.cpp=bin/%.o)
TARGET = bin/main

# Build the final executable
$(TARGET): $(OBJ)
	@$(CXX) $(CXXFLAGS) $^ -o $@

# Compile object files
bin/%.o: src/%.cpp
	@mkdir -p bin
	@$(CXX) $(CXXFLAGS) -c $< -o $@

# Run the program
run: $(TARGET)
	@./$(TARGET)   # <-- hides ./bin/main

# Clean
clean:
	@rm -rf bin
