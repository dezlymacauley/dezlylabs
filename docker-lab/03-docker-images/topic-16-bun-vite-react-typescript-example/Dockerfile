# SECTION: Build Steps

FROM oven/bun:1.2.2-alpine

WORKDIR /app/

COPY ./package.json ./
COPY ./bun.lock ./

# This will use `Bun` to install the project dependencies 
# that are listed in  `package.json`

# `--exact` will ensure that the exact version of the dependencies are listed.
RUN bun install --exact

# This will copy all of the files and directories into the working directory 
# of the container. With exception of those listed in the `dockerignore` file.
COPY ./ ./

# This will run the `build` script from the `package.json` file
# "build": "tsc -b && vite build",
RUN bun run build

#______________________________________________________________________________
