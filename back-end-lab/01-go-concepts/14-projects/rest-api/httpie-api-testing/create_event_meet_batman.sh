#!/usr/bin/env bash

# To run this file, cd to the httpie-api-testing directory, then run:
# chmod +x create_event.sh

# Run the program:
# go run .

# Then open a separate terminal and type this:
# ./create_event.sh

# The casing of the field names does not matter for JSON

# dateTime must be in ISO format
http POST 127.0.0.1:8080/events \
Content-Type:application/json \
<<'EOF'
{
    "name": "Meet Batman ",
    "description": "Meet the Dark Knight",
    "location": "Goham City",
    "dateTime": "2025-01-01T15:30:00.000Z"
}
EOF
