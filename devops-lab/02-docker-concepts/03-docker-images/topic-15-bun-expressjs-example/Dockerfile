# Download this image from Docker Hub, or the local Docker image cache
# on the computer that is running this command.
FROM oven/bun:1.3.0-alpine

# This will set the root directory of a container that is built from this
# image to `/app/`
WORKDIR /app/

# Copy only JavaScript files needed to install the project dependencies
# because these files are usually not meant to change.

# Copy package.json from this directory to the WORKDIR of the container.
# When a container is built, this file will be found in /app/package.json
COPY ./package.json ./

# Same concept her
COPY ./bun.lock ./

# Install the project dependencies and prevent any changes to `bun.lock` 
RUN bun install --frozen-lockfile

# When a container is built, this src/ directory will 
# be found in /app/src/ of the docker container.
COPY ./src/ ./src/

# An environment variable called PORT is declared and set to 3000
ENV PORT=3000

CMD [ "bun", "src/main.ts"]
