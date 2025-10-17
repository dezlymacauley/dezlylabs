# This will use version 23 of Node.js as the base image
FROM node:23

# Sets the working directory inside the container to /app
# All commands and paths will be relative to this directory
WORKDIR /app

# This will copy the `package.json` and `package-lock.json` files
# from your local directory, to the working directory of the container.
COPY package.json ./
COPY package-lock.json ./

# This will install dependencies using a clean install
RUN npm ci

# This will copy the source code to the working directory of container
# So `src/` of your local directory will be located in `/app/src/`
COPY src/ ./src/

# NOTE: Always copy your source code AFTER you have installed 
# your dependencies. 
# Docker builds an image using a layered (aka step by step approach).
# If you make changes to a line, Docker has to re-execute all 
# of the commands below it.

# So you don't want Docker to re-install dependencies any time you change 
# your source code.
 
# This will expose port 3000 of the container so that you can access 
# the container from your machine once the container has been built.
EXPOSE 3000

# Start the server by running `npm run start`
# This command is run from the container's working directory (`/app`).
# And `start` is actually a script in `package.json`
CMD ["npm", "run", "start"]
