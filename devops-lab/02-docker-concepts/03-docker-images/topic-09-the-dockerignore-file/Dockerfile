FROM node:25.0.0-alpine3.21

WORKDIR /app

# This will copy everything from this local directory
# into the working directory of the container
# Files listed in `.dockerignore` will not be added.
COPY ./ ./

CMD [ "node", "index.js" ]
