#!/usr/bin/env bash

# To run this file, cd to the api-testing directory, then run:
# chmod +x get_event.sh

# Run the program:
# go run .

# Then open a separate terminal and type this:
# ./get_event.sh.sh

http GET 127.0.0.1:8080/events
